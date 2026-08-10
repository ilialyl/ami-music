use anyhow::Result;
pub mod mpris;

const COVER_PORT: &str = "7878";
const DAEMON_PORT: &str = "7878";

pub fn local_ip_addr() -> Result<String> {
    Ok(local_ip_address::local_ip()?.to_string())
}

pub fn cover_addr() -> Result<String> {
    Ok(format!("{}:{}", local_ip_addr()?, COVER_PORT))
}

pub fn daemon_addr() -> Result<String> {
    Ok(format!("{}:{}", local_ip_addr()?, DAEMON_PORT))
}
