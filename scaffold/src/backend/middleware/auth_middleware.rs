use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error, http::header, Error, HttpResponse,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use serde_json::json;
use std::rc::Rc;
use tracing::{error, info};
use actix_web::body::BoxBody;

use crate::backend::utils::jwt::verify_and_renew_jwt;

// 忽略的路径: /register /ping
const IGNORED_PATHS: [&str; 3] = ["/register", "/ping", "/code"];

fn is_ignored_path(path: &str) -> bool {
    IGNORED_PATHS.iter().any(|&ignored| path.starts_with(ignored))
}

fn extract_token_from_headers(req: &ServiceRequest) -> String {
    req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header_str| {
            header_str
                .strip_prefix("Bearer ")
                .or_else(|| header_str.strip_prefix("bearer "))
        })
        .unwrap_or("")
        .to_string()
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let path = req.path().to_string();

        // 如果是跳过验证的路径，直接继续处理
        if is_ignored_path(&path) {
            return Box::pin(svc.call(req));
        }

        // 提取 token 并进行验证
        let token = extract_token_from_headers(&req);
        if token.is_empty() {
            return Box::pin(async move {
                Err(error::ErrorUnauthorized(json!({
                    "msg": "token不能为空",
                    "code": 2,
                    "path": path
                })))
            });
        }

        Box::pin(async move {
            match verify_and_renew_jwt(&token) {
                Ok(new_token) => {
                    // 如果 token 被续签了，添加到响应头
                    let mut response = svc.call(req).await?;
                    response.headers_mut().insert(
                        header::AUTHORIZATION,
                        header::HeaderValue::from_str(&format!("Bearer {}", new_token)).unwrap(),
                    );
                    Ok(response)
                }
                Err(err) => {
                    error!("JWT verification failed: {:?}", err);
                    Err(error::ErrorUnauthorized(json!({
                        "msg": "无效的token",
                        "code": 2,
                        "path": path
                    })))
                }
            }
        })
    }
}