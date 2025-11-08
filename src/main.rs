mod ui;
mod version;

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "clipboard", about = "Win+V style clipboard history for Linux", version = version::VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Toggle popup (shortcut convenience) if passed as a flag
    #[arg(long)]
    toggle: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the background daemon (clipboard monitor)
    Daemon,
    /// Show (or start) the UI popup
    Popup,
    /// Print version information
    Version,
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let cli = Cli::parse();

    if cli.toggle {
        ui::toggle_popup();
        return Ok(());
    }

    match cli.command.unwrap_or(Commands::Popup) {
        Commands::Daemon => run_daemon().await?,
        Commands::Popup => ui::toggle_popup(),
        Commands::Version => {
            println!("clipboard {}", version::VERSION);
        }
    }
    Ok(())
}

async fn run_daemon() -> anyhow::Result<()> {
    tracing::info!("Starting daemon (stub)");
    // TODO: Initialize backend + IPC + store.
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
