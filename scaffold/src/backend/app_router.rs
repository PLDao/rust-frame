use actix_web::{App, HttpServer, web, middleware, http, Responder, HttpResponse, Scope};
use actix_cors::Cors;
use sea_orm::DbConn;
use tracing::info;
use crate::backend::AppState;
use crate::backend::middleware::auth_middleware::Auth;
use crate::backend::middleware::time::Timed;
use crate::backend::api::auth::auth_scope;
// use crate::backend::api::password::password_scope;
// use crate::backend::api::admin::admin_scope;
// use crate::backend::api::logs::logs_scope;
use crate::backend::api::code::code_scope;
use crate::backend::api::qr_login::qr_login_scope;

pub async fn run_backend_server(
    pg_client: DbConn,
    backend_port: u16,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                      .allow_any_origin()
                      .allow_any_header()
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
            .route("/ping", web::get().to(router_hello))
            .service(auth_scope())     // 用户认证 API
            .service(code_scope())     // 验证码 API
            .service(qr_login_scope()) // 扫码登录 API
    })
        .bind(("0.0.0.0", backend_port))?
        .run()
        .await
}

pub async fn router_hello() -> impl Responder {
    info!("Hello World");
    HttpResponse::Ok().body("Pong")
}