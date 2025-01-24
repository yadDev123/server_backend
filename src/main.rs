use axum::{
    routing::post,
    Json, Router,
    serve,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use reqwest::Client;

#[derive(Deserialize)]
struct Payload {
    message: String,
}

#[derive(Serialize)]
struct DiscordPayload {
    content: String,
}

async fn send_to_discord(Json(payload): Json<Payload>) -> &'static str {
    let webhook_url = "YOUR_DISCORD_WEBHOOK_URL"; // Replace with your Discord webhook
    let client = Client::new();
    
    let discord_payload = DiscordPayload {
        content: payload.message,
    };

    if let Err(e) = client.post(webhook_url)
        .json(&discord_payload)
        .send()
        .await
    {
        eprintln!("Failed to send message: {}", e);
        return "Error sending message to Discord";
    }

    "Message sent to Discord"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/send", post(send_to_discord));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()) // ✅ Correct `axum::serve`
        .await
        .unwrap();
}
