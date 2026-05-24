use ami_core::cache::get_cover_art_cache_path;
use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub mod mpris;

const COVER_PORT: &str = "7879";
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

pub fn run_cover_art_service() -> Result<()> {
    let cover_art_dir_service =
        Router::new().fallback_service(ServeDir::new(get_cover_art_cache_path()?));

    let cover_addr = cover_addr()?;
    tokio::spawn(async {
        axum::serve(
            TcpListener::bind(cover_addr).await.unwrap(),
            cover_art_dir_service,
        )
        .await
        .unwrap();
    });

    Ok(())
}
