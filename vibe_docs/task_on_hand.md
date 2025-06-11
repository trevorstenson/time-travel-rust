# 🎯 Current Task: Time Travel Debugger for JavaScript

_Started: 2025-01-27_
_Status: 🚀 SETUP PHASE - Environment Setup & Initial Development_

## 📋 PROJECT UNDERSTANDING

### Project Name
**Time Travel Debugger for JavaScript** (using deno_core)

### Project Description
A toy implementation of a time-travel debugger for JavaScript to learn how these kinds of debuggers work. The debugger will:
- Execute JavaScript code using deno_core (V8 engine)
- Hook into execution state in a Rust program
- Capture snapshots of execution state
- Allow navigation backward and forward through execution history
- Provide debugging interface to inspect variables at any point in time

### Target Users
- **Primary**: Learning/educational use for understanding time-travel debugging concepts
- **Secondary**: Potential foundation for more advanced debugging tools

### Success Criteria
- [ ] Successfully execute JavaScript code via deno_core in Rust
- [ ] Capture execution state snapshots at function boundaries
- [ ] Navigate backward/forward through execution timeline
- [ ] Inspect variable values at any captured point
- [ ] Demonstrate working time-travel debugging on simple JavaScript programs

## 🔧 TECHNICAL REQUIREMENTS

### Technology Stack
- **Core Language**: Rust 1.80.0+
- **JavaScript Engine**: deno_core 0.311+ (V8 bindings)
- **Async Runtime**: tokio (for event loop)
- **State Storage**: In-memory for MVP, potentially SQLite later
- **Interface**: CLI for MVP, potential web interface later

### Architecture Components
1. **JavaScript Execution Engine**: deno_core runtime integration
2. **State Capture System**: Hook into V8 execution to record snapshots
3. **Time Navigation**: Forward/backward movement through execution
4. **State Storage**: Efficient storage of execution snapshots
5. **Debug Interface**: CLI for querying state at any point

### Development Environment
- **OS**: macOS 24.5.0 (development), cross-platform target
- **Tools**: Rust/Cargo, VS Code, Git
- **Dependencies**: deno_core, tokio, serde, anyhow

## ❓ OPEN QUESTIONS

### High Priority Questions (ANSWERED)
1. ✅ What is the main problem this project aims to solve?
   **Answer**: Learn how time-travel debuggers work by building a toy implementation

2. ✅ Who are the primary users and what are their key needs?
   **Answer**: Educational/learning use - understanding time-travel debugging concepts

3. ✅ What does success look like for this project?
   **Answer**: Working time-travel debugger that can step through JavaScript execution

4. ✅ Are there any hard technical requirements or constraints?
   **Answer**: Must use deno_core, must be in Rust, must hook into execution state

5. ✅ What is the expected timeline for completion?
   **Answer**: Learning project - no hard deadline, focus on learning and experimentation

### Technical Questions (ANSWERED)
1. ✅ Do you have preferences for programming languages?
   **Answer**: Rust for the debugger core, JavaScript for the code being debugged

2. ✅ Do you have preferences for frameworks or libraries?
   **Answer**: deno_core for JavaScript execution, tokio for async runtime

3. ✅ Will this need to integrate with existing systems?
   **Answer**: No, standalone learning project

4. ✅ What are the deployment requirements?
   **Answer**: Local development, cross-platform binary

5. ✅ Are there specific security or compliance needs?
   **Answer**: No special requirements for learning project

### Implementation Questions (TO BE EXPLORED)
1. How do we hook into deno_core's execution to capture state changes?
2. What granularity of state capture is needed (function calls, line by line, expressions)?
3. How do we efficiently store and retrieve execution snapshots?
4. What's the best way to represent JavaScript variable state in Rust?
5. How do we handle async JavaScript execution in our time-travel model?

## 🏗️ IMPLEMENTATION PLAN

### Phase 1: Foundation - Basic JavaScript Execution Engine ✅ (COMPLETED)

#### 1.1 JavaScript Runtime Setup ✅ (COMPLETED)
- [x] Research time-travel debugging concepts and approaches
- [x] Research deno_core integration patterns
- [x] Create environment setup documentation
- [x] Initialize Rust project structure with dependencies
- [x] Create comprehensive architectural plan
- [x] Implement basic deno_core runtime initialization
- [x] Add JavaScript file loading and execution
- [x] Implement basic console API (`console.log`, `console.error`, `console.warn`, `console.info`)
- [x] Create CLI for running JavaScript files
- [x] Add error handling and logging
- [x] Test with example JavaScript programs

**✅ SUCCESS CRITERIA MET**:
- ✅ Execute JavaScript files reliably (`hello_world.js`, `basic_functions.js`)
- ✅ Console output works correctly (all console methods functional)
- ✅ Basic error handling functional
- ✅ Simple function calls work perfectly
- ✅ CLI accepts JavaScript file paths and options
- ✅ Verbose mode shows execution statistics

#### 1.2 Basic Execution Monitoring ✅ (COMPLETED)
- [x] Implement function entry/exit detection using V8 hooks
- [x] Log execution flow to console with detailed context
- [x] Capture basic execution metadata (timestamps, function names, arguments)
- [x] Create structured execution trace output
- [x] Add execution context tracking (call stack depth, scope info)
- [x] Implement function call counting and statistics

**✅ SUCCESS CRITERIA MET**:
- ✅ Function entry/exit detection working (`op_function_entry` and `op_function_exit`)
- ✅ Execution flow logging with timestamps and duration tracking
- ✅ Context capture system functional (`op_capture_execution_context`)
- ✅ Nested function call monitoring with proper indentation
- ✅ Manual instrumentation API available in JavaScript (`timeDebugger` global)
- ✅ Basic execution metadata capture (function names, execution duration)

**🔧 IMPLEMENTATION DETAILS**:
- **V8 Operations**: Added `op_function_entry`, `op_function_exit`, `op_capture_execution_context`
- **JavaScript API**: Enhanced `timeDebugger` global object with monitoring capabilities
- **Execution Tracking**: Function call timing with millisecond precision
- **Context Capture**: Structured data capture for execution state
- **Nested Monitoring**: Proper handling of nested function calls

**🎯 DEMO RESULTS**:
- Manual monitoring test demonstrates all functionality working correctly
- Function calls properly tracked with entry/exit logs
- Execution timing captured (e.g., "testFunction (0.024ms)")
- Context data successfully captured and serialized
- Nested function calls maintain proper call hierarchy

### Phase 2: State Capture Foundation (FUTURE)
#### 2.1 JavaScript Value Serialization
- [ ] Implement JavaScript value → Rust conversion
- [ ] Handle primitive types (number, string, boolean, null, undefined)
- [ ] Basic object and array serialization
- [ ] Function metadata capture
- [ ] Circular reference detection and handling

#### 2.2 Execution Context Capture
- [ ] Capture local variable values at function boundaries
- [ ] Record complete call stack information
- [ ] Store precise source code locations
- [ ] Track global state changes
- [ ] Implement snapshot indexing system

### Phase 3: Time Navigation System (FUTURE)
#### 3.1 Snapshot Storage and Indexing
- [ ] Implement efficient snapshot storage
- [ ] Create timeline indexing system
- [ ] Add navigation primitives (next, previous, jump)
- [ ] Implement snapshot search capabilities
- [ ] Add memory management for large histories

### Phase 4: State Restoration Engine (FUTURE)
#### 4.1 V8 State Restoration
- [ ] Implement variable value restoration in V8
- [ ] Reconstruct call stack state
- [ ] Restore global object properties
- [ ] Handle scope chain reconstruction

### Phase 5: Interactive Debugging Interface (FUTURE)
#### 5.1 Command Line Interface Enhancement
- [ ] Implement interactive debugging REPL
- [ ] Add variable inspection commands
- [ ] Create execution timeline visualization

#### 5.2 Terminal-Based Visual Debugger (NEW CORE GOAL)
- [ ] Implement TUI (Terminal User Interface) for visual debugging
- [ ] Display source code in terminal with syntax highlighting
- [ ] Show ASCII arrow pointing to current execution line
- [ ] Real-time execution state display (call stack, variables, timing)
- [ ] Navigate through execution timeline with visual feedback
- [ ] Mimic professional debugger experience in terminal
- [ ] Support stepping through code (forward/backward in time)
- [ ] Show execution statistics and performance metrics in sidebar

**🎯 VISUALIZER GOALS:**
- **Source Code Display**: Show the actual JavaScript source with line numbers
- **Execution Pointer**: ASCII arrow (`-->`) indicating current paused line
- **State Panel**: Display current variables, call stack, and execution context
- **Timeline Navigation**: Visual representation of execution history
- **Interactive Controls**: Step forward/back, jump to specific points, set breakpoints
- **Performance Metrics**: Real-time display of function timing and call counts

**📺 EXAMPLE VISUALIZER LAYOUT:**
```
┌─ Source Code ─────────────────────────────┬─ Execution State ─────────┐
│  15 | function calculateFactorial(n) {     │ Call Stack:                │
│  16 |     if (n <= 1) {                   │ └─ main()                  │
│→ 17 |         return 1;                   │ └─ calculateFactorial(3)   │
│  18 |     }                               │                            │
│  19 |     return n * calculateFactorial(  │ Variables:                 │
│  20 | }                                   │ n: 1                       │
│                                          │ result: undefined          │
├─ Timeline ───────────────────────────────┤                            │
│ [●●●●●○○○○○] Step 5/10                   │ Timing:                    │
│ ← Step Back    Step Forward →           │ Function: 0.023ms          │
└─────────────────────────────────────────┴────────────────────────────┘
```

### Phase 6: Performance and Advanced Features (FUTURE)
- [ ] Performance optimization
- [ ] Advanced debugging features

## 📊 PROGRESS TRACKING

### Setup Phase (Current)
- [x] Environment setup documentation created
- [x] Project requirements understood
- [x] Technical stack chosen
- [ ] Basic Rust project initialized
- [ ] deno_core runtime working
- [ ] First JavaScript execution test

### Development Phase
- [ ] Basic JavaScript execution implemented
- [ ] State capture system designed
- [ ] Time navigation implemented
- [ ] CLI interface created
- [ ] Testing with sample programs

## 📝 DECISIONS MADE

### Project Decisions
- **2025-01-27**: Project scope - toy implementation for learning
- **2025-01-27**: Target language - JavaScript (executed via deno_core)
- **2025-01-27**: Implementation language - Rust
- **2025-01-27**: Primary goal - educational/learning tool

### Technical Decisions
- **2025-01-27**: Use deno_core 0.311+ for JavaScript execution
- **2025-01-27**: Use tokio for async runtime and event loop
- **2025-01-27**: Start with in-memory state storage for MVP
- **2025-01-27**: CLI interface for initial implementation

### Architecture Decisions
- **2025-01-27**: Hook into deno_core execution for state capture
- **2025-01-27**: Snapshot-based approach for time travel
- **2025-01-27**: Function-level granularity for initial implementation

## 🚀 NEXT STEPS

1. **Immediate**: Initialize basic Rust project with deno_core
2. **Then**: Get basic JavaScript execution working
3. **Next**: Research V8 hooks for state inspection
4. **Following**: Implement basic state capture mechanism
5. **Finally**: Build time navigation and CLI interface

## 🔗 REFERENCES

### Research Materials
- [deno_core Documentation](https://docs.rs/deno_core/latest/deno_core/)
- [Time Travel Debugging Concepts](https://blog.replay.io/how-to-build-a-time-machine)
- [FireDBG for Rust](https://github.com/SeaQL/FireDBG.for.Rust) - Reference implementation
- [Roll your own JavaScript runtime](https://deno.com/blog/roll-your-own-javascript-runtime)

### Technical References
- [V8 JavaScript Engine](https://v8.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

---

**Note**: This is an active learning project. The implementation approach may evolve as we discover more about deno_core's capabilities and time-travel debugging techniques.
