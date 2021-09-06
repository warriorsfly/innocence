use std::{cell::RefCell, pin::Pin, rc::Rc};

use actix_web::dev::{Service, Transform};
use futures::{
    future::{ok, Ready},
    Future,
};

pub struct InocAuth;

impl<S, Req> Transform<S, Req> for InocAuth
where
    S: Service<Req> + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type InitError = S::Error;
    type Transform = InocAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(InocAuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}
pub struct InocAuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, Req> Service<Req> for InocAuthMiddleware<S>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&self, req: Req) -> Self::Future {
        todo!()
    }
}
