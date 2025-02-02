use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tokio;
use reqwest::Client;
use base64::{encode, decode};

#[derive(Deserialize)]
struct A1 {
    a2: String,
}

#[derive(Serialize)]
struct B1 {
    b2: String,
}

fn c1(d1: &str) -> String {
    String::from_utf8(decode(d1).unwrap()).unwrap()
}

fn e1(f1: &str) -> String {
    encode(f1)
}

async fn g1() -> &'static str {
    "SGVsbG8sIFdvcmxkIQ=="
}

async fn h1(Json(i1): Json<A1>) -> &'static str {
    let j1 = "aHR0cHM6Ly9kaXNjb3JkLmNvbS9hcGkvd2ViaG9va3MvMTMzMjgwMTM4OTQ2MTYzNTEzMi9iU1NZdkgwcWxXeGdoVWpYaXdMbFpfbE1tWXdQZ3RvVXZ6Ni0tdWFNTnZUbXR5MkRjQ2hXUmNFYUcwRnd2eGR1eEIydA==";
    let k1 = c1(j1);

    if i1.a2.contains("@everyone") || i1.a2.contains("@here") {
        eprintln!("x1");
        return "y1";
    }
    
    let l1 = Client::new();
    let m1 = B1 { b2: i1.a2 };

    match l1.post(&k1).json(&m1).send().await {
        Ok(n1) if n1.status().is_success() => "o1",
        Ok(n1) => {
            eprintln!("p1: {}", n1.status());
            "q1"
        }
        Err(r1) => {
            eprintln!("s1: {}", r1);
            "t1"
        }
    }
}

#[tokio::main]
async fn u1() {
    let v1 = true;
    if v1 {
        println!("w1");
    } else {
        println!("x2");
    }
    
    let y2 = Router::new()
        .route("/a3", get(g1))
        .route("/b3", post(h1));
    
    let z2 = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let a4: SocketAddr = format!("0.0.0.0:{}", z2).parse().expect("b4");
    
    println!("c4 http://{}", a4);
    
    let d4 = tokio::net::TcpListener::bind(a4).await.expect("e4");
    axum::serve(d4, y2.into_make_service()).await.expect("f4");
}
