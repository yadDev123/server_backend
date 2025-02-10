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
// use axum::{routing::{get, post}, Json, Router};
// use serde::{Deserialize, Serialize};
// use std::env;
// use std::net::SocketAddr;
// use tokio;
// use reqwest::Client;
// use tracing::{info, error};
// use tracing_subscriber;

// #[derive(Deserialize)]
// struct Payload {
//     message: String,
// }

// #[derive(Serialize)]
// struct DiscordPayload {
//     content: String,
// }

// // Route for /test
// async fn test_handler() -> &'static str {
//     "Hello, World!"
// }

// // Handles sending messages to Discord
// async fn send_to_discord(Json(payload): Json<Payload>) -> &'static str {
//     let webhook_url = "https://discord.com/api/webhooks/1332801389461635132/bSSYvH0qlWxghUjXiwLlZ_lMmYwPgtoUvz6--uaMNvTmty2DcChWRcEaG0FwvxduxB2t";

//     // Log the received message
//     info!("Received message: {}", payload.message);

//     // Block messages containing @everyone or @here
//     if payload.message.contains("@everyone") || payload.message.contains("@here") {
//         error!("Blocked message containing @everyone or @here");
//         return "Blocked message: contains @everyone or @here";
//     }
    
//     let client = Client::new();
//     let discord_payload = DiscordPayload {
//         content: payload.message.clone(),
//     };

//     // Send the message to Discord
//     match client.post(webhook_url)
//         .json(&discord_payload)
//         .send()
//         .await
//     {
//         Ok(response) if response.status().is_success() => {
//             info!("Message successfully sent to Discord");
//             "Message sent to Discord"
//         }
//         Ok(response) => {
//             error!("Discord API error: {}", response.status());
//             "Error sending message to Discord"
//         }
//         Err(e) => {
//             error!("Request error: {}", e);
//             "Error sending request to Discord"
//         }
//     }
// }

// #[tokio::main]
// async fn main() {
//     // Initialize logging
//     tracing_subscriber::fmt().init();
    
//     info!("Starting server...");

//     let app = Router::new()
//         .route("/test", get(test_handler))
//         .route("/send", post(send_to_discord));
    
//     let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
//     let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

//     info!("Server running at http://{}", addr);

//     let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind port");
//     axum::serve(listener, app.into_make_service())
//         .await
//         .expect("Server crashed");
// }
use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr};
use reqwest::Client;
use tracing::{info, error};
use tracing_subscriber;

#[derive(Deserialize)]
struct Payload {
    message: String,
    username: String,
    user_id: String,
    timestamp: String,
    avatar_url: String, // Avatar URL for the user
}

#[derive(Serialize)]
struct DiscordEmbed {
    title: String,
    description: String,
    fields: Vec<EmbedField>,
    color: u32,
    timestamp: String,
    thumbnail: Option<Thumbnail>,
}

#[derive(Serialize)]
struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Serialize)]
struct Thumbnail {
    url: String,
}

#[derive(Serialize)]
struct DiscordPayload {
    content: String,
    embeds: Vec<DiscordEmbed>,
}

// Route for /test
async fn test_handler() -> &'static str {
    "Hello, World!"
}

// Handles sending messages to Discord
async fn send_to_discord(Json(payload): Json<Payload>) -> &'static str {
    let webhook_url = env::var("DISCORD_WEBHOOK_URL")
        .unwrap_or_else(|_| "https://discord.com/api/webhooks/your_webhook_url_here".to_string());

    // Log the received message
    info!("Received message: {}", payload.message);

    // Block messages containing @everyone or @here
    if payload.message.contains("@everyone") || payload.message.contains("@here") {
        error!("Blocked message containing @everyone or @here");
        return "Blocked message: contains @everyone or @here";
    }

    let client = Client::new();

    // Construct the Discord embed message
    let discord_embed = DiscordEmbed {
        title: format!("Message from {}", payload.username),
        description: payload.message.clone(),
        fields: vec![
            EmbedField {
                name: "User ID",
                value: payload.user_id.clone(),
                inline: true,
            },
            EmbedField {
                name: "Timestamp",
                value: payload.timestamp.clone(),
                inline: true,
            },
        ],
        color: 0x0099ff, // Blue color for the embed
        timestamp: payload.timestamp.clone(),
        thumbnail: if !payload.avatar_url.is_empty() {
            Some(Thumbnail {
                url: payload.avatar_url.clone(),
            })
        } else {
            None
        },
    };

    let discord_payload = DiscordPayload {
        content: format!("Message from {}:", payload.username),
        embeds: vec![discord_embed],
    };

    // Send the message to Discord
    match client.post(&webhook_url)
        .json(&discord_payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            info!("Message successfully sent to Discord");
            "Message sent to Discord"
        }
        Ok(response) => {
            error!("Discord API error: {}", response.status());
            "Error sending message to Discord"
        }
        Err(e) => {
            error!("Request error: {}", e);
            "Error sending request to Discord"
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt().init();

    info!("Starting server...");

    let app = Router::new()
        .route("/test", get(test_handler))
        .route("/send", post(send_to_discord));

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");

    info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind port");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server crashed");
}
