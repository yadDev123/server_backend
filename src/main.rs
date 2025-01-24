use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::env;
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
    let webhook_url = "https://discord.com/api/webhooks/1332416584156839996/uFyfp1H5vP8hxWwjBJpisON4vSOJO3OgVFJkapWlzbVFRSsy_htVi5F0eNGypyNN7IBL"; // Replace with your Discord webhook

    // 🔹 Check for forbidden mentions (@everyone or @here)
    if payload.message.contains("@everyone") || payload.message.contains("@here") {
        eprintln!("Blocked message containing @everyone or @here");
        return "Blocked message: contains @everyone or @here";
    }

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

    // 🔹 Render sets `PORT`, so we read it from the environment
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
