fn main(){
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable");
    let manager = r2d2_postgres::PostgresConnectionManager::new(
        database_url.parse().unwrap(), r2d2_postgres::postgres::NoTls);
    let pool = r2d2::Pool::new(manager).expect("initialize pool");
    println!("{:?}", pool);
}
