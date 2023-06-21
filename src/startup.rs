use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use std::net::TcpListener;

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(
  listener: TcpListener,
  db_pool: PgPool,
) -> Result<Server, std::io::Error> {
  let db_pool = Data::new(db_pool);
  let server = HttpServer::new(move || {
      App::new()
          // Middlewares are added using the `wrap` method on `App`
          .wrap(Logger::default())
          .route("/health_check", web::get().to(health_check))
          .route("/subscriptions", web::post().to(subscribe))
          // Register the connection as part of the application state
          .app_data(db_pool.clone())
  })
      .listen(listener)?
      .run();
  Ok(server)
}