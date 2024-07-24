// src/main.rs
mod handler;
mod routes;
mod model;
use warp::{http::Method, Filter};

#[tokio::main]
async fn main() {
    let cors = warp::cors()
    .allow_origin("http://localhost:3000")
    .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(vec!["Content-Type"]);

    println!("Starting server...");
    let routes = routes::routes().with(cors);
    println!("Server is running at http://localhost:3030"); 

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
