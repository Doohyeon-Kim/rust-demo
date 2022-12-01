use axum::{routing::get, Router};

fn main() {
    init();
}

#[tokio::main]
async fn init(){
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}
