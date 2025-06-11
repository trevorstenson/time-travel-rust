# Troubleshooting Guide

## Error Log Format
For each error, document:

### [Error Title/Type]
**Date:** [YYYY-MM-DD]
**Error Message:**
```
[Exact error message]
```

**Context:** [What were you trying to do?]
**Root Cause:** [Why did this happen?]
**Solution:**
1. [Step-by-step solution]
2. [Include exact commands used]

**Prevention:** [How to avoid this in the future]
**Related Files:** [Which files were modified to fix this]

---

## Documented Errors

### Import Error - deno_core::Error Not Found
**Date:** 2025-01-27
**Error Message:**
```
error[E0432]: unresolved import `deno_core::Error`
 --> src/runtime/engine.rs:1:60
  |
1 | use deno_core::{extension, op2, JsRuntime, RuntimeOptions, Error as CoreError};
  |                                                            -----^^^^^^^^^^^^^
  |                                                            |
  |                                                            no `Error` in the root
```

**Context:** Initial implementation of TimeDebuggerRuntime, trying to import deno_core::Error for error handling
**Root Cause:** deno_core doesn't export Error directly from the root module. The error types are in submodules or we should use anyhow::Result instead.
**Solution:**
1. Remove `Error as CoreError` from the import
2. Use `anyhow::Result` for error handling instead
3. Updated import to: `use deno_core::{extension, op2, JsRuntime, RuntimeOptions};`

**Prevention:** Check deno_core documentation for correct import paths. Use anyhow for general error handling.
**Related Files:** `src/runtime/engine.rs`

---

### Extension ESM Module Loading Error
**Date:** 2025-01-27
**Error Message:**
```
thread 'main' panicked at /Users/trevorstenson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/deno_core-0.311.0/runtime/jsruntime.rs:739:9:
Failed to initialize a JsRuntime: ext:time_debugger/runtime.js not present in the module map
```

**Context:** Trying to run JavaScript files with our custom extension but the JavaScript runtime module wasn't being loaded properly
**Root Cause:** Incorrect syntax for including JavaScript files in deno_core extensions. Multiple syntax attempts failed.
**Solution:**
1. Tried `esm = ["runtime.js" = include_str!("runtime.js")]` - syntax error
2. Tried `esm = [ "ext:time_debugger_extension/init.js" = include_str!("runtime.js") ]` - syntax error  
3. **Working solution**: `esm = [dir "src/runtime", "runtime.js"]` with proper entry point
4. Final working configuration:
```rust
extension!(
    time_debugger_extension,
    ops = [
        op_log_function_call,
        op_get_timestamp,
    ],
    esm_entry_point = "ext:time_debugger_extension/runtime.js",
    esm = [dir "src/runtime", "runtime.js"],
);
```

**Prevention:** Use the `[dir "directory", "file.js"]` syntax for including JavaScript files in deno_core extensions. Refer to deno_core examples and documentation.
**Related Files:** `src/runtime/engine.rs`, `src/runtime/runtime.js`

---

### Rust Installation Missing
**Date:** 2025-01-27
**Error Message:**
```
command not found: rustc
command not found: cargo
```

**Context:** Trying to initialize Rust project but Rust toolchain wasn't installed on the system
**Root Cause:** Rust wasn't installed on the development machine
**Solution:**
1. Install Rust using rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y`
2. Source the cargo environment: `source "$HOME/.cargo/env"`
3. Verify installation: `cargo --version && rustc --version`

**Prevention:** Always verify prerequisite installations before starting Rust projects. Check environment setup documentation.
**Related Files:** `vibe_docs/environment_setup.md`

---

## Common Patterns

### deno_core Extension Development
- Always use `[dir "directory", "filename"]` syntax for ESM files
- Entry point should match the directory structure: `ext:extension_name/filename`
- Test with simple JavaScript first before adding complex functionality
- Use `#[op2(fast)]` for synchronous operations, `#[op2(async)]` for async operations

### JavaScript Runtime Development
- Start with basic console API before adding complex features
- Test with progressively more complex JavaScript examples
- Use verbose mode (`--verbose`) for debugging execution issues
- Always handle both sync and async execution paths

### Error Handling Best Practices
- Use `anyhow::Result` for general error handling in Rust
- Provide descriptive error messages with context
- Always test error paths with invalid inputs
- Document common error patterns for future reference

### V8 String Size Limit Error (Runtime JS)
**Date:** 2025-01-27
**Error Message:**
```
error[E0080]: evaluation of constant value failed
evaluation panicked: assertion failed: buffer.is_ascii() && buffer.len() <= ((1 << 29) - 24)
note: inside `FastStaticString::create_external_onebyte_const`
```

**Context:** Attempting to build with enhanced JavaScript runtime file during Phase 1.2 development
**Root Cause:** V8 has a size limit for embedded string constants. Large JavaScript runtime files exceed this limit when included in the extension macro.
**Solution:**
1. Simplify the JavaScript runtime file to reduce size
2. Remove complex instrumentation code from the embedded runtime
3. Focus on core functionality in the embedded JS file
4. Move complex logic to Rust operations instead

**Prevention:** Keep embedded JavaScript files minimal and focused on essential APIs only
**Related Files:** `src/runtime/runtime.js`, `src/runtime/engine.rs`

### Op2 Fast Compatibility Error
**Date:** 2025-01-27
**Error Message:**
```
error: custom attribute panicked
This op is fast-compatible and should be marked as (fast)
```

**Context:** Adding new deno_core operations for function monitoring
**Root Cause:** Operations that only use simple types should be marked as `#[op2(fast)]` for better performance
**Solution:**
1. Use `#[op2(fast)]` for operations with simple parameters (strings, numbers)
2. Use `#[op2]` (non-fast) for operations with complex serde types
3. Check parameter types to determine appropriate annotation

**Prevention:** Review parameter types when creating new operations and choose appropriate op2 annotation
**Related Files:** `src/runtime/engine.rs` 