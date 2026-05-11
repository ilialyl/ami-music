use ami_core::cache::get_thumbnail_cache_path;
use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub mod mpris;

pub const COVER_ADDR: &str = "0.0.0.0:7879";

pub fn run_thumbnail_service() -> Result<()> {
    let cover_art_dir_service =
        Router::new().fallback_service(ServeDir::new(get_thumbnail_cache_path()?));

    tokio::spawn(async {
        axum::serve(
            TcpListener::bind(COVER_ADDR).await.unwrap(),
            cover_art_dir_service,
        )
        .await
        .unwrap();
    });

    Ok(())
}
