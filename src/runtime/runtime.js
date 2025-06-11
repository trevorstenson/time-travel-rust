// JavaScript runtime bridge for time travel debugger
(() => {
  const core = Deno.core;

  // Console API
  const console = {
    log(...args) {
      const message = args.map(arg => String(arg)).join(' ');
      core.print(`[LOG] ${message}\n`);
    },
    error(...args) {
      const message = args.map(arg => String(arg)).join(' ');
      core.print(`[ERROR] ${message}\n`, true);
    },
    warn(...args) {
      const message = args.map(arg => String(arg)).join(' ');
      core.print(`[WARN] ${message}\n`);
    },
    info(...args) {
      const message = args.map(arg => String(arg)).join(' ');
      core.print(`[INFO] ${message}\n`);
    }
  };

  // Time travel debugger API
  const timeDebugger = {
    logFunctionCall(functionName) {
      core.ops.op_log_function_call(functionName);
    },
    getTimestamp() {
      return core.ops.op_get_timestamp();
    },
    functionEntry(name) {
      core.ops.op_function_entry(name);
    },
    functionExit(name, durationMs) {
      core.ops.op_function_exit(name, durationMs);
    },
    captureContext(contextType, data) {
      core.ops.op_capture_execution_context(contextType, data);
    },
    // New value serialization API
    captureVariable(name, value) {
      try {
        return core.ops.op_capture_variable(name, value);
      } catch (e) {
        console.error(`Failed to capture variable ${name}:`, e);
        return null;
      }
    },
    captureScope(functionName, snapshotType, scopeObject) {
      try {
        return core.ops.op_capture_scope(functionName, snapshotType, scopeObject || {});
      } catch (e) {
        console.error(`Failed to capture scope for ${functionName}:`, e);
        return 0;
      }
    },
    getSnapshotInfo() {
      try {
        return core.ops.op_get_snapshot_info();
      } catch (e) {
        console.error('Failed to get snapshot info:', e);
        return null;
      }
    },
    // Enhanced function monitoring with variable capture
    captureFunction(fn, name) {
      if (typeof fn !== 'function') {
        throw new Error('First argument must be a function');
      }
      
      const functionName = name || fn.name || 'anonymous';
      
      return function(...args) {
        // Capture function entry with arguments
        timeDebugger.functionEntry(functionName);
        
        // Capture arguments as variables
        const argScope = {};
        args.forEach((arg, index) => {
          argScope[`arg${index}`] = arg;
        });
        timeDebugger.captureScope(functionName, 'entry', argScope);
        
        const startTime = timeDebugger.getTimestamp();
        let result;
        let error;
        
        try {
          result = fn.apply(this, args);
          
          // Capture return value
          if (result !== undefined) {
            timeDebugger.captureVariable('return', result);
          }
          
        } catch (e) {
          error = e;
          timeDebugger.captureVariable('error', e.toString());
        }
        
        const duration = (timeDebugger.getTimestamp() - startTime) * 1000;
        timeDebugger.functionExit(functionName, duration);
        
        if (error) {
          throw error;
        }
        
        return result;
      };
    }
  };

  // Make APIs globally available
  globalThis.console = console;
  globalThis.timeDebugger = timeDebugger;

  console.log('Time Travel Debugger Runtime v2.1 - JavaScript Value Serialization Enabled');
})(); 