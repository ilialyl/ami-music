use axum::{
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};

use crate::state::AppState;

async fn websocket_handler(
    ws: WebSocketUpgrade,
    state: axum::extract::State<AppState>, // Extract shared state
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: axum::extract::State<AppState>) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            match text.as_str() {
                "play" => {
                    let status = state.orchestrator.player.playback_status().as_str();
                    let _ = socket.send(Message::Text(status.into())).await;
                }
                _ => {
                    // Echo unknown commands
                    let _ = socket
                        .send(Message::Text(format!("Unknown command: {}", text).into()))
                        .await;
                }
            }
        }
    }
}
