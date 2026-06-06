use std::net::SocketAddr;

mod env;
mod pages;

#[tokio::main]
async fn main() {
    let config = env::Env::load();
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, pages::get_router()).await.unwrap();
}
