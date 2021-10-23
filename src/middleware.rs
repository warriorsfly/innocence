use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::pin::Pin;

use crate::{constants::MESSAGE_INVALID_TOKEN, errors, plugins::decode_jwt};

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware { service })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let _authorized = req.headers().get("Authorization");

        if let Some(_authorized) = _authorized {
            let _split: Vec<&str> = _authorized.to_str().unwrap().split("Bearer").collect();
            let token = _split[1].trim();
            match decode_jwt(token) {
                Ok(_) => {
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                }
                Err(_) => Box::pin(async move {
                    Err(errors::Error::Unauthorized(MESSAGE_INVALID_TOKEN.to_string()).into())
                }),
            }
        } else {
            Box::pin(async move {
                Err(errors::Error::Unauthorized(MESSAGE_INVALID_TOKEN.to_string()).into())
            })
        }
    }
}
