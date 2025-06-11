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
    }
  };

  // Make APIs globally available
  globalThis.console = console;
  globalThis.timeDebugger = timeDebugger;

  console.log('Time Travel Debugger Runtime v1.2 Enabled');
})(); 