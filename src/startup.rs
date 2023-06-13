use actix_web::{web, App , HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
      App::new()
          .route("/health_check", web::get().to(health_check))
          .route("/subscriptions", web::post().to(subscribe))
  })
      .listen(listener)?
      .run();
  Ok(server)
}