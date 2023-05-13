use axum::{http::StatusCode, routing::get, Router};

async fn handler() -> (StatusCode, String) {
    println!("Hello from handler");
    (StatusCode::OK, String::from("Hello reload"))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
