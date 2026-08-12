use ami_daemon::{
    app::App,
    cli::{Cli, CliCommand},
    daemon_process,
    logging::setup_logger,
};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        CliCommand::Start(args) => return daemon_process::handle_start(args.listen),
        CliCommand::Stop => return daemon_process::handle_stop(),
        CliCommand::Run(args) | CliCommand::Foreground(args) => {
            setup_logger()?;
            let app = App::new(args.listen)?;
            tokio::runtime::Runtime::new()?.block_on(app.run())
        }
    }
}
