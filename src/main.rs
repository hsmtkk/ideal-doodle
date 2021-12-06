use axum::{
    routing::get,
    Router,
};
use log::info;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::tokio_postgres::NoTls;

const TABLE_NAME: &str = "idurlmap";

#[tokio::main]
async fn main() {
    env_logger::init();

    let app_name = std::env::var("HEROKU_APP_NAME").expect("HEROKU_APP_NAME environment variable");
    let port = std::env::var("PORT").expect("PORT environment variable");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable");

    let manager = PostgresConnectionManager::new(
        database_url.parse().expect("parse DATABASE_URL"), NoTls);
    let pool = Pool::builder().build(manager).await.expect("init pool");

    init_database(pool.clone()).await;

    let bind_address = format!("0.0.0.0:{}", port);

    // build our application with a single route
    let app = Router::new()
        .layer(axum::AddExtensionLayer::new(pool.clone()))
        .route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&bind_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn init_database(pool: Pool<PostgresConnectionManager<NoTls>>){
    info!("init_database start");
    let mut client = pool.get().await.expect("get client");
    let find_table_sql = format!("SELECT COUNT(*) FROM {}", TABLE_NAME);
    match client.query(&find_table_sql, &[]).await{
        Ok(rows) => {
            info!("table exists");
        },
        Err(e) => {
            info!("table does not exist");
            let create_table_sql = format!("CREATE TABLE {} (id TEXT, url TEXT)", TABLE_NAME);
            client.execute(&create_table_sql,&[]).await.expect("create table");
            info!("created table");
        },
    }
    info!("init_database finish");
}