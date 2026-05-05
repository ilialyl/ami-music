use std::{fs, process, sync::Arc};

use ami_core::config::Config;
use ami_daemon::{
    internal_events::InternalEvent, logging::setup_logger, orchestrator::Orchestrator, services,
    states::new_shared_state, websockets::WebSocketService,
};
use anyhow::Result;
use tokio::{net::TcpListener, sync::broadcast};

// How many messages the broadcast channel can buffer
const CHANNEL_CAPACITY: usize = 32;
const DAEMON_ADDR: &str = "0.0.0.0:7878";
const PID_FILE: &str = "/tmp/ami_daemon.pid";

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("start") => return handle_start(),
        Some("stop") => return handle_stop(),
        Some("_run") | None => {}
        Some(other) => {
            eprintln!("Unknown command: {other}");
            return Ok(());
        }
    }
    setup_logger()?;
    tokio::runtime::Runtime::new()?.block_on(run_daemon())
}

fn handle_start() -> Result<()> {
    if let Ok(pid_str) = fs::read_to_string(PID_FILE) {
        let pid: i32 = pid_str.trim().parse().unwrap();
        let alive = unsafe { libc::kill(pid, 0) == 0 };
        if alive {
            eprintln!("Already running as {pid}");
            return Ok(());
        }
        let _ = fs::remove_file(PID_FILE);
    }

    let child = process::Command::new(std::env::current_exe()?)
        .arg("_run")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn()
        .unwrap();
    println!("Started PID {}", child.id());
    Ok(())
}

fn handle_stop() -> Result<()> {
    let pid: u32 = fs::read_to_string(PID_FILE)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    unsafe {
        libc::kill(pid as i32, libc::SIGTERM);
    }
    fs::remove_file(PID_FILE).unwrap();
    println!("Stopped {pid}");

    Ok(())
}

async fn run_daemon() -> Result<()> {
    log::debug!("Daemon starting...");

    let (internal_event_tx, internal_event_rx) =
        tokio::sync::mpsc::unbounded_channel::<InternalEvent>();

    let shared_state = new_shared_state(internal_event_tx.clone())?;
    let config = Config::load()?;

    shared_state
        .write()
        .await
        .orchestrator
        .library
        .load(config.library);

    services::run_thumbnail_service()?;

    let tx = internal_event_tx.clone();
    let player = Arc::clone(&shared_state.read().await.orchestrator.playback.player);
    tokio::spawn(async move { Orchestrator::send_player_position(player, tx).await });

    let listener = TcpListener::bind(DAEMON_ADDR).await?;
    log::debug!("Server listening on {DAEMON_ADDR}");

    // A broadcast channel: one sender, many receivers (one per client)
    let (connection_tx, _) = broadcast::channel::<String>(CHANNEL_CAPACITY);
    let connection_tx = Arc::new(connection_tx); // Share the sender across tasks

    let mut ws_service =
        WebSocketService::new(listener, connection_tx, internal_event_rx, shared_state);
    ws_service.start().await
}
