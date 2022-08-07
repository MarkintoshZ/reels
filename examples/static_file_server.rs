use reels::{router::Router, server::Server};

fn main() {
    Server::new(Router::new())
        .bind("127.0.0.1:8080".parse().unwrap())
        .start();
    println!("Listening on port 8080");
}
