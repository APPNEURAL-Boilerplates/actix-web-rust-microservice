use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

#[derive(Clone, Debug)]
pub struct RequestIdValue(pub String);

#[derive(Clone, Default)]
pub struct RequestId;

impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestIdMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = req
            .headers()
            .get(REQUEST_ID_HEADER)
            .and_then(|value| value.to_str().ok())
            .filter(|value| !value.trim().is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        req.extensions_mut()
            .insert(RequestIdValue(request_id.clone()));
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let mut response = service.call(req).await?;
            if let Ok(header_value) = HeaderValue::from_str(&request_id) {
                response
                    .headers_mut()
                    .insert(HeaderName::from_static(REQUEST_ID_HEADER), header_value);
            }
            Ok(response)
        })
    }
}
