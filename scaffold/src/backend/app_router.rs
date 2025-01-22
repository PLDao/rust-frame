use actix_web::{App, HttpServer, web, middleware, http, Responder, HttpResponse};
use actix_cors::Cors;
use sea_orm::DbConn;
use tracing::info;
use crate::backend::api::code::code_scope;
use crate::backend::AppState;
use crate::backend::middleware::auth_middleware::Auth;
use crate::backend::middleware::time::Timed;

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
            .wrap(Timed)
            .wrap(Auth)
            .app_data(web::Data::new(AppState { pg_client: pg_client.clone() }))
            .route("/ping", web::get().to(router_hello)) // 添加这个路由
            .service(code_scope())

    })
        .bind(("0.0.0.0", backend_port))?
        .run()
        .await
}


pub async fn router_hello() -> impl Responder {
    info!("Hello World");
    HttpResponse::Ok().body("Pong")
}