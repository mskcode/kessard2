use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use futures_util::future::LocalBoxFuture;
use slog::{info, Logger};

pub struct LoggingMiddlewareFactory {
    logger: Logger,
}

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddleware {
            service: service,
            logger: self.logger.clone(),
        }))
    }
}

impl LoggingMiddlewareFactory {
    pub fn new(logger: &Logger) -> Self {
        LoggingMiddlewareFactory {
            logger: logger.clone(),
        }
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
    logger: Logger,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let endpoint = format!("{} {}", req.method(), req.path());
        info!(self.logger, "req-start: {}", endpoint);

        let fut = self.service.call(req);

        // there could be a way to get self.logger inside the closure below
        // with some lifetime tricks but I'm too newb for those yet
        let logger = self.logger.clone();

        Box::pin(async move {
            let res = fut.await?;
            info!(logger, "req-end: {} {}", endpoint, res.status().as_u16());
            Ok(res)
        })
    }
}
