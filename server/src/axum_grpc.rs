use std::{convert::Infallible, task::{Context, Poll}};
use std::error::Error as StdError;
use axum::body::{Body as AxumBody, Bytes, HttpBody};
use axum::http::header::CONTENT_TYPE;
use axum::http::{Request, Response, StatusCode};
use axum::response::Response as AxumResponse;
use tonic::codegen::BoxFuture;
use tower::{Service, ServiceExt};

fn is_grpc(req: &Request<AxumBody>) -> bool {
    req.headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.starts_with("application/grpc"))
        .unwrap_or(false)
}

fn to_axum_body<B>(body: B) -> AxumBody
where
    B: HttpBody<Data = Bytes> + Send + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>> + 'static
{
    AxumBody::new(body)
}

fn internal_error<E: std::fmt::Display>(err: E) -> AxumResponse {
    let mut r = AxumResponse::new(AxumBody::from(format!("internal error: {}", err)));
    *r.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    r
}

#[derive(Clone, Debug)]
pub struct ContentTypeSwitch<G, H> {
    grpc: G,
    http: H,
}

impl<G, H> ContentTypeSwitch<G, H> {
    pub fn new(grpc: G, http: H) -> Self {
        Self { grpc, http }
    }
}

impl<G, GB, H, HB> Service<Request<AxumBody>> for ContentTypeSwitch<G, H>
where
// gRPC Service Bounds
    G: Service<Request<AxumBody>, Response = Response<GB>, Error = Infallible> + Clone + Send + 'static,
    G::Future: Send + 'static,
    GB: HttpBody<Data = Bytes> + Send + 'static,
// FIX: Explicitly require GB::Error to be thread-safe and an Error
    GB::Error: StdError + Send + Sync + 'static,

// HTTP Service Bounds
    H: Service<Request<AxumBody>, Response = Response<HB>, Error = Infallible> + Clone + Send + 'static,
    H::Future: Send + 'static,
    HB: HttpBody<Data = Bytes> + Send + 'static,
// FIX: Explicitly require HB::Error to be thread-safe and an Error
    HB::Error: StdError + Send + Sync + 'static,
{
    type Response = AxumResponse;
    type Error = Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<AxumBody>) -> Self::Future {
        let is_grpc_req = is_grpc(&req);
        let grpc = self.grpc.clone();
        let http = self.http.clone();

        Box::pin(async move {
            if is_grpc_req {
                match grpc.oneshot(req).await {
                    Ok(res) => {
                        let (parts, body) = res.into_parts();
                        // Because we bounded GB::Error to Send + Sync + StdError,
                        // it now satisfies the Into<Box<...>> bound in to_axum_body
                        Ok(Response::from_parts(parts, to_axum_body(body)))
                    }
                    Err(e) => Ok(internal_error(e)),
                }
            } else {
                match http.oneshot(req).await {
                    Ok(res) => {
                        let (parts, body) = res.into_parts();
                        Ok(Response::from_parts(parts, to_axum_body(body)))
                    }
                    Err(e) => Ok(internal_error(e)),
                }
            }
        })
    }
}

pub type TonicCompatible<G, H> = ContentTypeSwitch<G, H>;