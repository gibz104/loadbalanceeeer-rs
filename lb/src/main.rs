// Import necessary libraries and modules
use poem::{web::Json, Endpoint, Route, Server, handler, listener::TcpListener};
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use rand::seq::SliceRandom;

// Define the proxy handler
#[handler]
async fn proxy_handler(
    Json(json): Json<Value>,  // JSON payload received from client
) -> poem::Result<Json<Value>> {
    // Read the RPCS environment variable, split by commas, and map to a Vec of Strings
    let rpcs = env::var("RPCS")
        .unwrap()
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();

    // Define the SOCKS5 proxy URL
    let proxy = "socks5://tor-socks-proxy:9150".to_string();

    // Choose a random RPC from the list
    let random_rpc = rpcs.choose(&mut rand::thread_rng()).unwrap().clone();

    // Build the HTTP client with the SOCKS5 proxy setting
    let client = Client::builder().proxy(reqwest::Proxy::all(proxy).unwrap()).build().unwrap();

    // Get the client's IP address (verify its a tor exit node)
    let ip_address = get_ip_address(client.clone()).await;
    println!("Using RPC {} with IP {}", random_rpc, ip_address);

    // Send the proxy request to the random RPC
    let proxy_req = client.post(&random_rpc)
        .json(&json)
        .send()
        .await
        .unwrap();

    // Get the response from the RPC and parse it as JSON
    let response = proxy_req.json::<Value>().await.unwrap();

    // Return the response as a Result
    Ok(Json(response))
}

// Function to get the client's IP address
async fn get_ip_address(client: Client) -> String {
    // Send a GET request to ipify's API to get the IP address
    let proxy_request = client.get("https://api.ipify.org?format=json")
        .send()
        .await
        .unwrap();

    // Parse the response as JSON
    let response = proxy_request.json::<Value>().await.unwrap();

    // Extract the IP address from the response and return it as a String
    response["ip"].as_str().unwrap().to_string()
}

#[tokio::main]
async fn main() {
    // Define the route for the application
    let app = Route::new().at("/", proxy_handler);

    // Start the server
    Server::new(TcpListener::bind("0.0.0.0:9545"))
        .run(app)
        .await.unwrap();
}
