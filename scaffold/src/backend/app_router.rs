use actix_web::{App, HttpServer, web, middleware, http};
use actix_cors::Cors;
use sea_orm::DbConn;
use crate::backend::AppState;
use crate::backend::middleware::auth_middleware::Auth;

pub async fn run_backend_server(
    pg_client: DbConn,
    backend_port: u16,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                      .allowed_origin("http://127.0.0.1:3000")
                      .allowed_origin("http://localhost:3000")
                      .send_wildcard()
                      .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                      .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                      .allowed_header(http::header::CONTENT_TYPE)
                      .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .wrap(Auth)
            .app_data(web::Data::new(AppState { pg_client: pg_client.clone() }))
        // .service(transactions_scope())
        // .service(block_scope())
        // .default_service(web::route().to(not_found))
    })
        .bind(("0.0.0.0", backend_port))?
        .run()
        .await
}

