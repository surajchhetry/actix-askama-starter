use std::future::{ready, Ready};

use actix_web::{dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse};
use askama::Template;
use futures_util::future::LocalBoxFuture;

#[derive(Template)]
#[template(path = "errors/500.html")]
pub struct InternalServerErrorTemplate;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct InternalServerError;

// Middleware factory is `Transform` trait from actix-service crate
impl<S> Transform<S, ServiceRequest> for InternalServerError
    where
        S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
        S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = InternalServerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InternalServerMiddleware { service }))
    }
}

pub struct InternalServerMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for InternalServerMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
        S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            match fut.await {
                Ok(mut res) if res.response().status().is_server_error() => {
                    let s = InternalServerErrorTemplate.render().unwrap();
                    let error_response = HttpResponse::InternalServerError().content_type("text/html").body(s);
                    *res.response_mut() = error_response;
                    Ok(res)
                },
                other_result => other_result,
            }
        })
    }
}
