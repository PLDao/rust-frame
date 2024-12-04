use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use actix_web::http::header;
use futures_util::future::{ready, LocalBoxFuture, Ready};
use actix_web::{error};
use serde_json::json;
use std::rc::Rc;
use tracing::{error, info};
use crate::backend::utils::jwt::{verify_and_renew_jwt};  // 引入你之前提供的 JWT 处理函数

// 中间件定义
pub struct Auth;

// 中间件工厂
impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
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
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let path = req.path().to_string();

        // 判断是否为注册接口，跳过 JWT 验证
        if path.contains("/register") {
            let fut = svc.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        // JWT 验证逻辑
        let def = header::HeaderValue::from_str("").unwrap();
        let token = req
            .headers()
            .get("Authorization")
            .unwrap_or(&def)
            .to_str()
            .ok()
            .unwrap()
            .replace("Bearer ", "");

        Box::pin(async move {
            info!("Requesting path: {}", path);

            // 如果 token 为空，返回认证错误
            if token.is_empty() {
                let error_response = json!({
                    "msg": "token不能为空",
                    "code": 2,
                    "path": path
                });
                return Err(error::ErrorUnauthorized(error_response.to_string()));
            }

            // 使用 `verify_and_renew_jwt` 验证并续期 JWT
            let jwt_verification = verify_and_renew_jwt(&token);

            match jwt_verification {
                Ok(token_data) => {
                    // 不再进行路径权限的检查，直接继续调用下一个服务
                    let fut = svc.call(req);
                    let res = fut.await?;
                    Ok(res)
                }
                Err(err) => {
                    error!("JWT validation failed: {}", err);
                    let error_response = json!({
                        "msg": "无效的token",
                        "code": 2,
                        "path": path
                    });
                    Err(error::ErrorUnauthorized(error_response.to_string()))
                }
            }
        })
    }
}