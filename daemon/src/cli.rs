use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,

}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Start the daemon in the background
    Start(StartArgs),
    /// Stop the running daemon
    Stop,
    #[command(hide = true)]
    Run(StartArgs),
    /// Start in foreground
    Foreground(StartArgs),
}

#[derive(Args)]
pub struct StartArgs {
    #[arg(long)]
    pub listen: bool,
}
