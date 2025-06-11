use deno_core::{extension, op2, JsRuntime, RuntimeOptions, OpState, v8};
use std::rc::Rc;
use std::cell::RefCell;
use anyhow::Result;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use crate::runtime::serialization::{JSValue, SerializationContext, SerializationConfig};

/// Configuration for the time travel debugger
#[derive(Debug, Clone)]
pub struct DebuggerConfig {
    pub capture_enabled: bool,
    pub max_snapshots: usize,
    pub verbose: bool,
    pub trace_function_calls: bool,
}

impl Default for DebuggerConfig {
    fn default() -> Self {
        Self {
            capture_enabled: true,
            max_snapshots: 1000,
            verbose: false,
            trace_function_calls: true,
        }
    }
}

/// Function call information for execution monitoring
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub timestamp: f64,
    pub call_depth: usize,
    pub arguments: Vec<String>,
    pub file_location: Option<String>,
    pub line_number: Option<u32>,
}

/// Variable capture snapshot for a specific execution point
#[derive(Debug, Clone)]
pub struct VariableSnapshot {
    pub timestamp: f64,
    pub function_name: String,
    pub call_depth: usize,
    pub variables: HashMap<String, JSValue>,
    pub snapshot_type: String, // "entry", "exit", "custom"
}

/// Enhanced execution state tracking with function monitoring and variable capture
#[derive(Debug)]
pub struct ExecutionState {
    pub function_calls: u64,
    pub total_execution_time: std::time::Duration,
    pub current_function: Option<String>,
    pub call_stack_depth: usize,
    pub function_call_history: Vec<FunctionCall>,
    pub function_call_counts: HashMap<String, u32>,
    pub execution_start_time: Option<Instant>,
    pub variable_snapshots: Vec<VariableSnapshot>,
    pub serialization_context: SerializationContext,
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self {
            function_calls: 0,
            total_execution_time: std::time::Duration::default(),
            current_function: None,
            call_stack_depth: 0,
            function_call_history: Vec::new(),
            function_call_counts: HashMap::new(),
            execution_start_time: None,
            variable_snapshots: Vec::new(),
            serialization_context: SerializationContext::new(SerializationConfig::default()),
        }
    }
}

impl ExecutionState {
    pub fn start_execution(&mut self) {
        self.execution_start_time = Some(Instant::now());
    }

    pub fn log_function_entry(&mut self, name: String, args: Vec<String>, location: Option<String>, line: Option<u32>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        self.function_calls += 1;
        self.call_stack_depth += 1;
        self.current_function = Some(name.clone());

        // Update function call counts
        let count = self.function_call_counts.entry(name.clone()).or_insert(0);
        *count += 1;

        // Store function call details
        let call = FunctionCall {
            name: name.clone(),
            timestamp,
            call_depth: self.call_stack_depth,
            arguments: args,
            file_location: location,
            line_number: line,
        };

        self.function_call_history.push(call);

        // Print execution trace
        let indent = "  ".repeat(self.call_stack_depth.saturating_sub(1));
        println!("📞 {}→ {} (depth: {}, count: {})", 
                indent, name, self.call_stack_depth, count);
    }

    pub fn log_function_exit(&mut self, name: String, duration_ms: f64) {
        if self.call_stack_depth > 0 {
            self.call_stack_depth -= 1;
        }

        let indent = "  ".repeat(self.call_stack_depth);
        println!("📤 {}← {} ({}ms)", indent, name, duration_ms);

        // Update current function to the parent if we have call history
        if let Some(parent_call) = self.function_call_history
            .iter()
            .rev()
            .find(|call| call.call_depth == self.call_stack_depth) {
            self.current_function = Some(parent_call.name.clone());
        } else {
            self.current_function = None;
        }
    }

    pub fn capture_variables(
        &mut self,
        scope: &mut v8::HandleScope,
        function_name: String,
        snapshot_type: String,
        variables: HashMap<String, v8::Local<v8::Value>>,
    ) -> Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let mut captured_vars = HashMap::new();
        
        // Serialize each variable using our serialization context
        for (name, value) in variables {
            match self.serialization_context.serialize_value(scope, value) {
                Ok(serialized_value) => {
                    captured_vars.insert(name, serialized_value);
                },
                Err(e) => {
                    // If serialization fails, store an error representation
                    captured_vars.insert(name, JSValue::Error(format!("Serialization failed: {}", e)));
                }
            }
        }

        let snapshot = VariableSnapshot {
            timestamp,
            function_name: function_name.clone(),
            call_depth: self.call_stack_depth,
            variables: captured_vars,
            snapshot_type: snapshot_type.clone(),
        };

        self.variable_snapshots.push(snapshot);

        // Print capture info if verbose
        let indent = "  ".repeat(self.call_stack_depth);
        println!("📸 {}CAPTURE: {} ({} - {} variables)", 
            indent, function_name, snapshot_type, 
            self.variable_snapshots.last().unwrap().variables.len());

        Ok(())
    }

    pub fn get_execution_trace(&self) -> String {
        let mut trace = String::new();
        trace.push_str("🔍 EXECUTION TRACE:\n");
        trace.push_str(&format!("Total function calls: {}\n", self.function_calls));
        trace.push_str(&format!("Variable snapshots: {}\n", self.variable_snapshots.len()));
        trace.push_str(&format!("Max call depth reached: {}\n", 
            self.function_call_history.iter().map(|c| c.call_depth).max().unwrap_or(0)));
        
        trace.push_str("\n📊 FUNCTION CALL STATISTICS:\n");
        let mut sorted_counts: Vec<_> = self.function_call_counts.iter().collect();
        sorted_counts.sort_by_key(|(_, count)| *count);
        sorted_counts.reverse();

        for (func_name, count) in sorted_counts.iter().take(10) {
            trace.push_str(&format!("  {} → {} calls\n", func_name, count));
        }

        trace.push_str("\n🕐 FUNCTION CALL TIMELINE:\n");
        for (i, call) in self.function_call_history.iter().enumerate().take(20) {
            let indent = "  ".repeat(call.call_depth.saturating_sub(1));
            trace.push_str(&format!("  {}: {}{}({})\n", 
                i + 1, indent, call.name, call.arguments.join(", ")));
        }

        if self.function_call_history.len() > 20 {
            trace.push_str(&format!("  ... and {} more calls\n", 
                self.function_call_history.len() - 20));
        }

        // Add variable snapshot summary
        if !self.variable_snapshots.is_empty() {
            trace.push_str("\n📸 VARIABLE SNAPSHOTS:\n");
            for (i, snapshot) in self.variable_snapshots.iter().enumerate().take(10) {
                let indent = "  ".repeat(snapshot.call_depth);
                trace.push_str(&format!("  {}: {}{} [{}] - {} vars\n", 
                    i + 1, indent, snapshot.function_name, 
                    snapshot.snapshot_type, snapshot.variables.len()));
            }
            if self.variable_snapshots.len() > 10 {
                trace.push_str(&format!("  ... and {} more snapshots\n", 
                    self.variable_snapshots.len() - 10));
            }
        }

        trace
    }
}

/// Main time travel debugger runtime
pub struct TimeDebuggerRuntime {
    js_runtime: JsRuntime,
    execution_state: Rc<RefCell<ExecutionState>>,
    config: DebuggerConfig,
}

impl TimeDebuggerRuntime {
    /// Create a new time travel debugger runtime
    pub fn new(config: DebuggerConfig) -> Result<Self> {
        let execution_state = Rc::new(RefCell::new(ExecutionState::default()));
        
        let mut js_runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![time_debugger_extension::init_ops_and_esm()],
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            ..Default::default()
        });

        // Put the execution state in op state so ops can access it
        js_runtime.op_state().borrow_mut().put(execution_state.clone());

        Ok(Self {
            js_runtime,
            execution_state,
            config,
        })
    }

    /// Execute a JavaScript file
    pub async fn execute_file(&mut self, file_path: &str) -> Result<()> {
        self.execution_state.borrow_mut().start_execution();
        
        if self.config.verbose {
            println!("🎯 Loading JavaScript file: {}", file_path);
            println!("🔧 Monitoring configuration:");
            println!("   - Function call tracing: {}", self.config.trace_function_calls);
            println!("   - State capture: {}", self.config.capture_enabled);
            println!("   - Max snapshots: {}", self.config.max_snapshots);
        }

        // Resolve the file path
        let main_module = deno_core::resolve_path(
            file_path, 
            &std::env::current_dir()?
        )?;

        // Load and evaluate the main module
        let mod_id = self.js_runtime.load_main_es_module(&main_module).await?;
        let result = self.js_runtime.mod_evaluate(mod_id);
        
        // Run the event loop to completion
        self.js_runtime.run_event_loop(Default::default()).await?;
        result.await?;

        // Update execution statistics
        {
            let mut execution_state = self.execution_state.borrow_mut();
            if let Some(start_time) = execution_state.execution_start_time {
                execution_state.total_execution_time = start_time.elapsed();
            }
        }
        
        if self.config.verbose {
            let execution_state = self.execution_state.borrow();
            println!("✅ Execution completed in {:?}", execution_state.total_execution_time);
            println!("\n{}", execution_state.get_execution_trace());
        }

        Ok(())
    }

    /// Get current execution state for debugging
    pub fn get_execution_state(&self) -> &Rc<RefCell<ExecutionState>> {
        &self.execution_state
    }
}

// Custom operations for the time travel debugger
#[op2(fast)]
fn op_log_function_call(_state: &mut OpState, #[string] function_name: String) {
    println!("📞 Function called: {}", function_name);
}

#[op2(fast)]  
fn op_get_timestamp() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

#[op2(fast)]
fn op_function_entry(state: &mut OpState, #[string] name: String) {
    println!("🔍 ENTER: {}", name);
    
    // Update the execution state
    if let Some(execution_state) = state.try_borrow_mut::<Rc<RefCell<ExecutionState>>>() {
        execution_state.borrow_mut().log_function_entry(name, vec![], None, None);
    }
}

#[op2(fast)]
fn op_function_exit(state: &mut OpState, #[string] name: String, duration_ms: f64) {
    println!("🔍 EXIT:  {} ({}ms)", name, duration_ms);
    
    // Update the execution state
    if let Some(execution_state) = state.try_borrow_mut::<Rc<RefCell<ExecutionState>>>() {
        execution_state.borrow_mut().log_function_exit(name, duration_ms);
    }
}

#[op2]
fn op_capture_execution_context(
    #[string] context_type: String,
    #[serde] data: serde_json::Value
) {
    println!("📸 CONTEXT: {} -> {:?}", context_type, data);
}

/// Capture a single variable value for serialization
#[op2]
#[string]
fn op_capture_variable(
    scope: &mut v8::HandleScope,
    state: &mut OpState,
    #[string] variable_name: String,
    value: v8::Local<v8::Value>,
) -> Result<String, anyhow::Error> {
    if let Some(execution_state) = state.try_borrow_mut::<Rc<RefCell<ExecutionState>>>() {
        let mut exec_state = execution_state.borrow_mut();
        
        // Serialize the value
        match exec_state.serialization_context.serialize_value(scope, value) {
            Ok(serialized_value) => {
                let display_str = serialized_value.to_display_string();
                println!("📝 Variable captured: {} = {}", variable_name, display_str);
                Ok(display_str)
            },
            Err(e) => {
                let error_msg = format!("Failed to serialize {}: {}", variable_name, e);
                println!("❌ {}", error_msg);
                Err(anyhow::anyhow!(error_msg))
            }
        }
    } else {
        Err(anyhow::anyhow!("Execution state not available"))
    }
}

/// Capture multiple variables at once (e.g., function arguments or local scope)
#[op2(fast)]
fn op_capture_scope(
    scope: &mut v8::HandleScope,
    state: &mut OpState,
    #[string] function_name: String,
    #[string] snapshot_type: String,
    scope_object: v8::Local<v8::Value>,
) -> u32 {
    if let Some(execution_state) = state.try_borrow_mut::<Rc<RefCell<ExecutionState>>>() {
        let mut exec_state = execution_state.borrow_mut();
        let mut variables = HashMap::new();

        // If the scope_object is an object, extract its properties
        if scope_object.is_object() {
            if let Ok(object) = v8::Local::<v8::Object>::try_from(scope_object) {
                if let Some(property_names) = object.get_own_property_names(scope, v8::GetPropertyNamesArgs::default()) {
                    let length = property_names.length();
                    
                    for i in 0..length {
                        if let Some(key) = property_names.get_index(scope, i) {
                            let key_string = key.to_rust_string_lossy(scope);
                            
                            if let Some(property_value) = object.get(scope, key) {
                                variables.insert(key_string, property_value);
                            }
                        }
                    }
                }
            }
        }

        let var_count = variables.len() as u32;
        
        // Capture the variables using the execution state method
        if let Err(e) = exec_state.capture_variables(scope, function_name, snapshot_type, variables) {
            println!("❌ Failed to capture variables: {}", e);
            return 0;
        }
        
        var_count
    } else {
        println!("❌ Execution state not available");
        0
    }
}

/// Get information about captured snapshots
#[op2]
#[serde]
fn op_get_snapshot_info(state: &mut OpState) -> Result<serde_json::Value, anyhow::Error> {
    if let Some(execution_state) = state.try_borrow::<Rc<RefCell<ExecutionState>>>() {
        let exec_state = execution_state.borrow();
        
        let snapshot_info = serde_json::json!({
            "total_snapshots": exec_state.variable_snapshots.len(),
            "function_calls": exec_state.function_calls,
            "call_depth": exec_state.call_stack_depth,
            "current_function": exec_state.current_function,
            "snapshots": exec_state.variable_snapshots.iter().take(5).map(|snapshot| {
                serde_json::json!({
                    "timestamp": snapshot.timestamp,
                    "function": snapshot.function_name,
                    "type": snapshot.snapshot_type,
                    "depth": snapshot.call_depth,
                    "variable_count": snapshot.variables.len()
                })
            }).collect::<Vec<_>>()
        });
        
        Ok(snapshot_info)
    } else {
        Err(anyhow::anyhow!("Execution state not available"))
    }
}

// Extension definition with enhanced debugging APIs
extension!(
    time_debugger_extension,
    ops = [
        op_log_function_call,
        op_get_timestamp,
        op_function_entry,
        op_function_exit,
        op_capture_execution_context,
        op_capture_variable,
        op_capture_scope,
        op_get_snapshot_info,
    ],
    esm_entry_point = "ext:time_debugger_extension/runtime.js",
    esm = [dir "src/runtime", "runtime.js"],
); 