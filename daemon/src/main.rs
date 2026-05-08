use ami_daemon::{daemon_process, logging::setup_logger, states::App};
use anyhow::Result;

// How many messages the broadcast channel can buffer

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("start") => return daemon_process::handle_start(),
        Some("stop") => return daemon_process::handle_stop(),
        Some("_run") | None => {}
        Some(other) => {
            eprintln!("Unknown command: {other}");
            return Ok(());
        }
    }
    setup_logger()?;
    let app = App::new()?;
    tokio::runtime::Runtime::new()?.block_on(app.run())
}
