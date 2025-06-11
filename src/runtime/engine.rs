use deno_core::{extension, op2, JsRuntime, RuntimeOptions};
use std::rc::Rc;
use anyhow::Result;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

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

/// Enhanced execution state tracking with function monitoring
#[derive(Debug, Default)]
pub struct ExecutionState {
    pub function_calls: u64,
    pub total_execution_time: std::time::Duration,
    pub current_function: Option<String>,
    pub call_stack_depth: usize,
    pub function_call_history: Vec<FunctionCall>,
    pub function_call_counts: HashMap<String, u32>,
    pub execution_start_time: Option<Instant>,
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

    pub fn get_execution_trace(&self) -> String {
        let mut trace = String::new();
        trace.push_str("🔍 EXECUTION TRACE:\n");
        trace.push_str(&format!("Total function calls: {}\n", self.function_calls));
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

        trace
    }
}

/// Main time travel debugger runtime
pub struct TimeDebuggerRuntime {
    js_runtime: JsRuntime,
    execution_state: ExecutionState,
    config: DebuggerConfig,
}

impl TimeDebuggerRuntime {
    /// Create a new time travel debugger runtime
    pub fn new(config: DebuggerConfig) -> Result<Self> {
        let js_runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![time_debugger_extension::init_ops_and_esm()],
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            ..Default::default()
        });

        Ok(Self {
            js_runtime,
            execution_state: ExecutionState::default(),
            config,
        })
    }

    /// Execute a JavaScript file
    pub async fn execute_file(&mut self, file_path: &str) -> Result<()> {
        self.execution_state.start_execution();
        
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
        if let Some(start_time) = self.execution_state.execution_start_time {
            self.execution_state.total_execution_time = start_time.elapsed();
        }
        
        if self.config.verbose {
            println!("✅ Execution completed in {:?}", self.execution_state.total_execution_time);
            println!("\n{}", self.execution_state.get_execution_trace());
        }

        Ok(())
    }

    /// Get current execution state for debugging
    pub fn get_execution_state(&self) -> &ExecutionState {
        &self.execution_state
    }
}

// Custom operations for the time travel debugger
#[op2(fast)]
fn op_log_function_call(#[string] function_name: String) {
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
fn op_function_entry(#[string] name: String) {
    println!("🔍 ENTER: {}", name);
}

#[op2(fast)]
fn op_function_exit(#[string] name: String, duration_ms: f64) {
    println!("🔍 EXIT:  {} ({}ms)", name, duration_ms);
}

#[op2]
fn op_capture_execution_context(
    #[string] context_type: String,
    #[serde] data: serde_json::Value
) {
    println!("📸 CONTEXT: {} -> {:?}", context_type, data);
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
    ],
    esm_entry_point = "ext:time_debugger_extension/runtime.js",
    esm = [dir "src/runtime", "runtime.js"],
); 