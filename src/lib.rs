pub mod configuration;
pub mod routes;
pub mod startup;

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use routes::{health_check, subscribe};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
