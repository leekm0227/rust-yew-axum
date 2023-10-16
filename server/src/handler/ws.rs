use axum::{
    extract::{ws::Message, State, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::stream::StreamExt;
use futures::SinkExt;

use crate::config::app::AppState;

pub async fn ws_handle(ws: WebSocketUpgrade, State(_state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        let (mut sender, mut receiver) = socket.split();

        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                match message.clone() {
                    Message::Text(message) => {
                        if sender.send(Message::Text(message)).await.is_err() {
                            break;
                        };
                    }
                    Message::Ping(ping) => {
                        if sender.send(Message::Pong(ping)).await.is_err() {
                            break;
                        };
                    }
                    Message::Pong(_) => {}
                    Message::Close(_) => {
                        break;
                    }
                    Message::Binary(_) => {}
                }
            }
        });

        // todo! server side message
        // let mut send_task = tokio::spawn(async move { todo!() });

        tokio::select! {
            // _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => tracing::info!("close")//send_task.abort(),
        }
    })
}
