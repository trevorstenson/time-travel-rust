# 🔧 Technical Details

_This document contains all technical decisions and implementation details._

## 💻 Technology Stack

### Core Runtime
- **Language**: Rust 1.80.0+ (latest stable)
- **JavaScript Engine**: deno_core 0.311+ (V8 bindings)
- **Async Runtime**: tokio 1.40+ (event loop management)
- **Build System**: Cargo (Rust package manager)

### Dependencies
- **deno_core**: JavaScript runtime integration
- **tokio**: Async runtime with full features
- **serde**: Serialization/deserialization for state storage
- **serde_json**: JSON handling for state representation
- **anyhow**: Error handling and context

### Development Tools
- **VS Code**: IDE with rust-analyzer extension
- **CodeLLDB**: Debugger for Rust development
- **cargo-watch**: Auto-rebuild during development
- **criterion**: Benchmarking (future use)

## 🏗️ Architecture

### System Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│                 │    │                 │    │                 │
│   CLI Interface │────│  Debugger Core  │────│ State Capture   │
│                 │    │   (Rust)        │    │   System        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                       ┌─────────────────┐
                       │                 │
                       │   deno_core     │
                       │  (V8 Runtime)   │
                       │                 │
                       └─────────────────┘
                                │
                       ┌─────────────────┐
                       │                 │
                       │  JavaScript     │
                       │  Execution      │
                       │                 │
                       └─────────────────┘
```

### Core Components

1. **CLI Interface**: User-facing command-line interface for debugging operations
2. **Debugger Core**: Main coordination logic, manages state and navigation
3. **State Capture System**: Hooks into V8 execution to record snapshots
4. **deno_core Runtime**: V8 JavaScript engine integration
5. **State Storage**: In-memory storage of execution snapshots

### Design Patterns
- **Command Pattern**: CLI commands for different debugging operations
- **Observer Pattern**: State change notifications from V8
- **Strategy Pattern**: Different state capture strategies
- **Repository Pattern**: State storage and retrieval abstraction

## 📁 Project Structure

### Directory Structure

```
time_travel_debugger/
├── src/
│   ├── main.rs              # Entry point and CLI parsing
│   │   ├── mod.rs
│   │   ├── engine.rs        # JavaScript runtime setup
│   │   └── ops.rs           # Custom operations
│   ├── debugger/            # Core debugger logic
│   │   ├── mod.rs
│   │   ├── core.rs          # Main debugger coordinator
│   │   ├── state.rs         # State management
│   │   └── navigation.rs    # Time travel navigation
│   ├── capture/             # State capture system
│   │   ├── mod.rs
│   │   ├── hooks.rs         # V8 execution hooks
│   │   └── snapshot.rs      # State snapshot logic
│   ├── storage/             # State storage
│   │   ├── mod.rs
│   │   ├── memory.rs        # In-memory storage
│   │   └── serialization.rs # State serialization
│   └── cli/                 # Command-line interface
│       ├── mod.rs
│       ├── commands.rs      # CLI command handlers
│       └── display.rs       # Output formatting
├── examples/                # JavaScript test programs
│   ├── simple.js
│   ├── functions.js
│   └── variables.js
├── tests/                   # Integration tests
├── docs/                    # Additional documentation
├── Cargo.toml               # Dependencies and metadata
└── README.md                # Project overview
```

### Key Modules

- **runtime**: deno_core integration and JavaScript execution
- **debugger**: Core time-travel debugging logic
- **capture**: State capture and snapshot mechanisms
- **storage**: State persistence and retrieval
- **cli**: User interface and command handling

## 🔒 State Capture Strategy

### Capture Points
- **Function Entry**: Before function execution begins
- **Function Exit**: After function execution completes
- **Variable Changes**: When variable values are modified (future)
- **Breakpoints**: User-defined stop points (future)

### State Representation
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionSnapshot {
    pub id: SnapshotId,
    pub timestamp: SystemTime,
    pub call_stack: Vec<StackFrame>,
    pub variables: HashMap<String, JSValue>,
    pub source_location: SourceLocation,
    pub execution_context: ExecutionContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function_name: String,
    pub file_path: String,
    pub line_number: u32,
    pub column_number: u32,
    pub local_variables: HashMap<String, JSValue>,
}
```

### V8 Integration Points
- **Function Call Hooks**: Intercept function invocations
- **Variable Access**: Monitor variable reads/writes
- **Scope Inspection**: Extract local/global variable values
- **Call Stack**: Capture current execution context

## 🚀 Development Setup

### Prerequisites
- Rust 1.80.0+ with cargo
- Git for version control
- VS Code with rust-analyzer (recommended)

### Build Configuration
```toml
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

### Environment Variables
- `RUST_LOG=debug`: Enable debug logging
- `RUST_BACKTRACE=1`: Full backtraces on panic
- `TIME_TRAVEL_DEBUG=true`: Enable additional debug output

## 📊 Performance Considerations

### State Storage Optimization
- **Incremental Snapshots**: Only store changed values
- **Compression**: Compress snapshot data for memory efficiency
- **Garbage Collection**: Remove old snapshots beyond history limit
- **Copy-on-Write**: Share unchanged data between snapshots

### Memory Management
- **Bounded History**: Limit number of stored snapshots
- **Lazy Loading**: Load snapshot data on demand
- **Weak References**: Avoid circular references in V8 integration
- **Resource Cleanup**: Proper cleanup of V8 resources

### Performance Targets (Initial)
- **Snapshot Creation**: < 10ms per snapshot
- **Navigation Speed**: < 100ms for forward/backward step
- **Memory Usage**: < 100MB for typical debugging session
- **Startup Time**: < 500ms for runtime initialization

## 🔍 Time Travel Implementation

### Navigation Model
```rust
pub struct TimeNavigator {
    snapshots: Vec<ExecutionSnapshot>,
    current_position: usize,
    history_limit: usize,
}

impl TimeNavigator {
    pub fn step_forward(&mut self) -> Result<&ExecutionSnapshot, NavigationError>;
    pub fn step_backward(&mut self) -> Result<&ExecutionSnapshot, NavigationError>;
    pub fn jump_to_position(&mut self, pos: usize) -> Result<&ExecutionSnapshot, NavigationError>;
    pub fn find_snapshot_by_criteria(&self, criteria: SearchCriteria) -> Option<usize>;
}
```

### State Restoration
- **Variable State**: Restore JavaScript variable values
- **Call Stack**: Reconstruct execution context
- **Program Counter**: Position in source code
- **Scope Chain**: Local and closure variable access

### Search and Navigation
- **Temporal Search**: Find snapshots by timestamp
- **Variable Search**: Find when variable had specific value
- **Function Search**: Find all calls to specific function
- **Condition Search**: Find snapshots matching conditions

## 🧪 Testing Strategy

### Unit Tests
- **State Capture**: Test snapshot creation and serialization
- **Navigation**: Test forward/backward movement
- **Storage**: Test state persistence and retrieval
- **V8 Integration**: Test JavaScript execution hooks

### Integration Tests
- **End-to-End**: Complete debugging sessions
- **JavaScript Programs**: Various JS patterns and features
- **Error Handling**: Recovery from execution errors
- **Performance**: Benchmark critical operations

### Test JavaScript Programs
```javascript
// examples/simple.js - Basic function calls
function add(a, b) {
    return a + b;
}

function main() {
    let x = 5;
    let y = 10;
    let result = add(x, y);
    console.log("Result:", result);
}

main();
```

## 🔄 Future Enhancements

### Short Term
- Expression-level state capture
- Conditional breakpoints
- Variable watch expressions
- Call stack visualization

### Medium Term
- Web-based debugging interface
- IDE integration (VS Code extension)
- Performance profiling integration
- Multi-threaded execution support

### Long Term
- Distributed system debugging
- Production environment integration
- Advanced visualization tools
- Machine learning-assisted debugging

## 📚 Research Areas

### V8 Engine Integration
- **Isolate Management**: V8 isolate lifecycle
- **Context Switching**: Multiple JavaScript contexts
- **Memory Management**: V8 garbage collector interaction
- **Performance Optimization**: Minimizing debugging overhead

### Time Travel Algorithms
- **Efficient State Diff**: Minimize storage requirements
- **Reverse Execution**: Algorithms for backward stepping
- **Branch Point Handling**: Multiple execution paths
- **Deterministic Replay**: Ensuring consistent reproduction

### JavaScript Runtime Internals
- **Scope Chain Management**: Variable resolution
- **Function Call Mechanics**: Call/return handling
- **Async Execution**: Promises, async/await debugging
- **Error Handling**: Exception propagation tracking

### Terminal Visualizer Architecture (Future)

#### TUI Framework Selection
- **ratatui**: Modern Rust TUI framework for rich terminal interfaces
- **crossterm**: Cross-platform terminal manipulation
- **syntect**: Syntax highlighting for source code display
- **unicode-width**: Proper text width calculation for layout

#### Visualizer Components
```
┌─ TUI Application ─────────────────────────────────────────┐
│ ┌─ Layout Manager ─────┬─ Source Display ─────────────────┐ │
│ │                      │ - Syntax highlighting           │ │
│ │ ┌─ State Panel ─────┐│ - Line numbers                  │ │
│ │ │ - Call Stack      ││ - Execution pointer (arrow)     │ │
│ │ │ - Variables       ││ - Scroll support                │ │
│ │ │ - Timing Info     │└─────────────────────────────────┘ │
│ │ └───────────────────┘                                   │
│ │ ┌─ Timeline ────────┐┌─ Controls ──────────────────────┐ │
│ │ │ - Progress bar    ││ - Step forward/back             │ │
│ │ │ - Current step    ││ - Jump to line                  │ │
│ │ │ - Total steps     ││ - Set breakpoints               │ │
│ │ └───────────────────┘└─────────────────────────────────┘ │
│ └───────────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────────┘
```

#### Source Code Mapping
```rust
#[derive(Debug, Clone)]
pub struct SourceMapping {
    pub file_path: String,
    pub content: String,
    pub lines: Vec<String>,
    pub line_to_execution_point: HashMap<usize, ExecutionPoint>,
}

#[derive(Debug, Clone)]
pub struct ExecutionPoint {
    pub snapshot_id: SnapshotId,
    pub function_name: String,
    pub timestamp: f64,
    pub call_depth: usize,
}
```

#### TUI State Management
```rust
pub struct DebuggerTUI {
    pub current_snapshot: Option<SnapshotId>,
    pub source_mapping: SourceMapping,
    pub viewport: SourceViewport,
    pub timeline_position: usize,
    pub execution_history: Vec<ExecutionSnapshot>,
}

pub struct SourceViewport {
    pub start_line: usize,
    pub end_line: usize,
    pub current_line: usize,
    pub arrow_position: (u16, u16),
}
```
