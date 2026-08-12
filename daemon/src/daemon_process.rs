use std::{fs, process};

use anyhow::Result;

use crate::services::{daemon_addr, daemon_addr_listen};

pub const PID_FILE: &str = "/tmp/ami_daemon.pid";

pub fn handle_start(listen: bool) -> Result<()> {
    if let Ok(pid_str) = fs::read_to_string(PID_FILE) {
        let pid: i32 = pid_str.trim().parse().unwrap();
        let alive = unsafe { libc::kill(pid, 0) == 0 };
        if alive {
            eprintln!("Already running as {pid}");
            return Ok(());
        }
        let _ = fs::remove_file(PID_FILE);
    }

    let mut cmd = process::Command::new(std::env::current_exe()?);
        cmd.arg("run");
        if listen {
            cmd.arg("--listen");
        }
        let child = cmd
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .spawn()?;

    println!("Started PID {}", child.id());
    let addr = if listen {
        daemon_addr_listen()?
    } else {
        daemon_addr()?
    };
    println!("Server listening on {}", addr);
    Ok(())
}

pub fn handle_stop() -> Result<()> {
    let pid: u32 = fs::read_to_string(PID_FILE)?.trim().parse()?;
    unsafe {
        libc::kill(pid as i32, libc::SIGTERM);
    }
    fs::remove_file(PID_FILE)?;
    println!("Stopped {pid}");

    Ok(())
}
