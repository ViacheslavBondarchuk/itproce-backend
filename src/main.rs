use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

mod constants;
mod environments;
mod services;
mod database;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    println!("{}", environments::web::parallelism());
    println!("{}", environments::database::url());
    println!("{}", environments::database::connection_pool_size());

    HttpServer::new(|| App::new()
        .default_service(web::to(services::default::not_implemented))
        .app_data(Data::new(database::connection_pool())))
        .workers(environments::web::parallelism())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
