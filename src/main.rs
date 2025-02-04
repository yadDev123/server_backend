use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tokio;
use reqwest::Client;

#[derive(Deserialize)]
struct Payload {
    webhook_message: String,
    dm_message: String,
    token: String, // User token instead of bot token
}

#[derive(Serialize)]
struct DiscordPayload {
    content: String,
}

async fn test_handler() -> &'static str {
    "Hello, World!"
}

async fn send_to_discord(Json(payload): Json<Payload>) -> &'static str {
    let webhook_url = "https://discord.com/api/webhooks/YOUR_WEBHOOK_URL"; // Replace with your actual webhook URL

    if payload.webhook_message.contains("@everyone") || payload.webhook_message.contains("@here") {
        eprintln!("Blocked message containing @everyone or @here");
        return "Blocked message: contains @everyone or @here";
    }
    
    let client = Client::new();
    let discord_payload = DiscordPayload {
        content: payload.webhook_message.clone(),
    };

    match client.post(webhook_url)
        .json(&discord_payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            println!("Message sent to Discord webhook. Now sending DMs...");
            tokio::spawn(send_dms(payload.token, payload.dm_message)); // Spawn task for DM sending
            "Message sent to Discord and DMs"
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

async fn send_dms(token: String, message: String) {
    let client = Client::new();
    let api_url = "https://discord.com/api/v10/users/@me/channels";

    let response = client.get(api_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            if let Ok(body) = resp.text().await {
                println!("Received DM channels: {}", body);
                
                match serde_json::from_str::<Vec<serde_json::Value>>(&body) {
                    Ok(dms) => {
                        for dm in dms {
                            if let Some(dm_id) = dm["id"].as_str() {
                                let msg_response = client.post(format!("https://discord.com/api/v10/channels/{}/messages", dm_id))
                                    .header("Authorization", format!("Bearer {}", token))
                                    .json(&serde_json::json!({"content": message}))
                                    .send()
                                    .await;

                                match msg_response {
                                    Ok(msg_resp) if msg_resp.status().is_success() => {
                                        println!("Message sent to DM: {}", dm_id);
                                    }
                                    Ok(msg_resp) => {
                                        eprintln!("Failed to send message to {}: {}", dm_id, msg_resp.status());
                                    }
                                    Err(e) => {
                                        eprintln!("Request error while sending DM to {}: {}", dm_id, e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to parse DM response: {}", e),
                }
            }
        }
        Ok(resp) => {
            let body = resp.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
            eprintln!("Failed to get DMs: {} - Response: {}", resp.status(), body);
        }
        Err(e) => eprintln!("Request error: {}", e),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/test", get(test_handler))
        .route("/send", post(send_to_discord));
    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    println!("\u{1F680} Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("❌ Failed to bind port");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("❌ Server crashed");
}
