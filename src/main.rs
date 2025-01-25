use axum::{routing::{get, post}, Json, Router, Server};
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

// ✅ Health check route for uptime monitoring (UptimeRobot, cron-job.org)
async fn health_check() -> &'static str {
    "✅ Server is running"
}

// ✅ Handles sending messages to Discord
async fn send_to_discord(Json(payload): Json<Payload>) -> &'static str {
    let webhook_url = "https://discord.com/api/webhooks/1332416584156839996/uFyfp1H5vP8hxWwjBJpisON4vSOJO3OgVFJkapWlzbVFRSsy_htVi5F0eNGypyNN7IBL"; // Replace with your actual webhook URL

    // 🔹 Block messages containing @everyone or @here to prevent spam
    if payload.message.contains("@everyone") || payload.message.contains("@here") {
        eprintln!("Blocked message containing @everyone or @here");
        return "Blocked message: contains @everyone or @here";
    }
    
    let client = Client::new();
    let discord_payload = DiscordPayload {
        content: payload.message,
    };

    // 🔹 Attempt to send the message to Discord
    match client.post(webhook_url)
        .json(&discord_payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            "Message sent to Discord"
        }
        Ok(response) => {
            eprintln!("Discord API error: {}", response.status());
            "Error sending message to Discord"
        }
        Err(e) => {
            eprintln!("Request error: {}", e);
            "Error sending request to Discord"
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check)) // ✅ Use this for UptimeRobot pings
        .route("/send", post(send_to_discord)); // 🔹 This is your main function for Discord messages
    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    println!("🚀 Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("❌ Server crashed");
}
