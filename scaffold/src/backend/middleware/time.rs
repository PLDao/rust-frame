use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error,
};
use futures_util::future::LocalBoxFuture;
use std::{future::{ready, Ready}, time::Instant};
use tracing::info;


pub struct Timed;

impl<S, B> Transform<S, ServiceRequest> for Timed
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TimedMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TimedMiddleware {
            service
        }))
    }
}

pub struct TimedMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TimedMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let path = req.path().to_owned();
        let method = req.method().to_string();
        let remote_addr = req.connection_info().peer_addr().unwrap_or("Unknown").to_string();
        let version = format!("{:?}", req.version());
        let headers = req.headers().clone();
        info!("{}","1. Pre-process the Request");
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            let elapsed = start_time.elapsed();
            let status = res.status();
            let content_length = res
                .headers()
                .get(actix_web::http::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("---");

            let user_agent = headers
                .get(actix_web::http::header::USER_AGENT)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("Unknown");
            info!("{}","2. Post-process a Response");
            info!("{} {} {} {} {} {} {} time took [{:.6}] ms",
                remote_addr,
                method,
                path,
                version,
                status.as_u16(),
                content_length,
                user_agent,
                elapsed.as_millis()
            );
            Ok(res)
        })
    }
}