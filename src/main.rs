use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app_name = std::env::var("HEROKU_APP_NAME").expect("HEROKU_APP_NAME environment variable");
    let port = std::env::var("PORT").expect("PORT environment variable");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable");
    let bind_address = format!("0.0.0.0:{}", port);

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&bind_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}