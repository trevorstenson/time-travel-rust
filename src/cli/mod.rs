use crate::runtime::{TimeDebuggerRuntime, DebuggerConfig};
use anyhow::{Result, anyhow};
use std::env;
use std::path::Path;

/// Command line interface for the time travel debugger
pub struct DebuggerCli {
    config: DebuggerConfig,
}

impl DebuggerCli {
    pub fn new() -> Self {
        Self {
            config: DebuggerConfig::default(),
        }
    }

    /// Parse command line arguments and run the debugger
    pub async fn run() -> Result<()> {
        let args: Vec<String> = env::args().collect();
        
        if args.len() < 2 {
            Self::print_help();
            return Ok(());
        }

        let mut cli = Self::new();
        let mut file_path = None;

        // Simple argument parsing
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--verbose" | "-v" => {
                    cli.config.verbose = true;
                },
                "--help" | "-h" => {
                    Self::print_help();
                    return Ok(());
                },
                "--version" => {
                    println!("Time Travel Debugger v0.1.0");
                    return Ok(());
                },
                "--max-snapshots" => {
                    if i + 1 < args.len() {
                        cli.config.max_snapshots = args[i + 1].parse()
                            .map_err(|_| anyhow!("Invalid max-snapshots value"))?;
                        i += 1;
                    } else {
                        return Err(anyhow!("--max-snapshots requires a value"));
                    }
                },
                "--no-capture" => {
                    cli.config.capture_enabled = false;
                },
                arg if !arg.starts_with('-') => {
                    file_path = Some(arg.to_string());
                },
                _ => {
                    return Err(anyhow!("Unknown argument: {}", args[i]));
                }
            }
            i += 1;
        }

        // Validate file path
        let file_path = file_path.ok_or_else(|| anyhow!("No JavaScript file specified"))?;
        
        if !Path::new(&file_path).exists() {
            return Err(anyhow!("File not found: {}", file_path));
        }

        // Create and run the debugger
        cli.execute_file(&file_path).await
    }

    /// Execute a JavaScript file with the debugger
    async fn execute_file(&self, file_path: &str) -> Result<()> {
        println!("🚀 Time Travel Debugger starting...");
        
        if self.config.verbose {
            println!("🔧 Configuration: {:?}", self.config);
        }

        let mut runtime = TimeDebuggerRuntime::new(self.config.clone())?;
        
        match runtime.execute_file(file_path).await {
            Ok(()) => {
                if self.config.verbose {
                    let state = runtime.get_execution_state();
                    println!("📊 Execution Statistics:");
                    println!("   - Function calls: {}", state.function_calls);
                    println!("   - Total time: {:?}", state.total_execution_time);
                }
                println!("✅ Execution completed successfully");
                Ok(())
            },
            Err(e) => {
                eprintln!("❌ Execution failed: {}", e);
                Err(e)
            }
        }
    }

    /// Print help information
    fn print_help() {
        println!("Time Travel Debugger v0.1.0");
        println!("A toy implementation of time-travel debugging for JavaScript");
        println!();
        println!("USAGE:");
        println!("    time_travel_debugger [OPTIONS] <file.js>");
        println!();
        println!("ARGS:");
        println!("    <file.js>    JavaScript file to execute and debug");
        println!();
        println!("OPTIONS:");
        println!("    -v, --verbose            Enable verbose output");
        println!("    -h, --help               Print help information");
        println!("        --version            Print version information");
        println!("        --max-snapshots N    Maximum number of snapshots to keep (default: 1000)");
        println!("        --no-capture         Disable state capture (run in normal mode)");
        println!();
        println!("EXAMPLES:");
        println!("    time_travel_debugger examples/basic.js");
        println!("    time_travel_debugger --verbose --max-snapshots 500 script.js");
        println!("    time_travel_debugger --no-capture fast_script.js");
    }
} 