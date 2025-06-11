mod runtime;
mod cli;

use cli::DebuggerCli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the debugger CLI and run
    DebuggerCli::run().await
}
