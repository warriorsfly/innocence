use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::pin::Pin;

use crate::errors;

// pub struct InocAuth;

// impl<S, Req> Transform<S, Req> for InocAuth
// where
//     S: Service<Req, Error = actix_web::Error> + 'static,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type InitError = ();
//     type Transform = InocAuthMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(InocAuthMiddleware {
//             service: Rc::new(RefCell::new(service)),
//         })
//     }
// }
// pub struct InocAuthMiddleware<S> {
//     service: Rc<RefCell<S>>,
// }

// impl<S, Req> Service<Req> for InocAuthMiddleware<S>
// where
//     S: Service<Req, Error = actix_web::Error>,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(
//         &self,
//         ctx: &mut core::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(ctx)
//     }

//     fn call(&self, req: Req) -> Self::Future {
//         todo!();
//     }
// }

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

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
    
        let mut authorized = false;
        if authorized {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(
                async move { Err(errors::Error::Unauthorized("invalid token".to_string()).into()) },
            )
        }
    }
}
