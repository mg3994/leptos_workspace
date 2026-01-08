use std::{convert::Infallible,task::{Context, Poll}};
use std::error::Error;
use axum:: body::Body as AxumBody;
use axum::body::{Bytes, HttpBody};
use axum::http::header::CONTENT_TYPE;
use axum::http::{Request, Response, StatusCode};
use axum::response::Response as AxumResponse;
use tonic::codegen::BoxFuture;
use tower::Service;

// FIX: Changed signature to take a reference &Request
fn is_grpc(req: &Request<AxumBody>) -> bool {
    req.headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|s| s.starts_with("application/grpc"))
}

fn to_axum_body<B>(body: B) -> AxumBody where B: axum::body::HttpBody<Data=axum::body::Bytes> + Send + 'static,
B:: Error: Into<Box<dyn std::error::Error + Send + Sync>> + 'static {
    AxumBody::new(body)
}

fn internal_error<E:std::fmt::Display>(err: E) -> AxumResponse {
    let mut r = AxumResponse::new(AxumBody::from(format!("internal error: {}", err)));
    *r.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    r
}

#[derive(Clone,Debug)]
pub struct ContentTypeSwitch<G,H> {
    grpc:G,
    http:H,
}
impl<G,H> ContentTypeSwitch<G,H> {
    pub fn new(grpc:G, http:H) -> Self {
        Self {grpc,http}
    }
}
impl<G,GB,H,HB> Service<Request<AxumBody>> for ContentTypeSwitch<G,H> where G:Service<Request<AxumBody>,Response=Response<GB>, Error = Infallible> + Clone + Send + 'static, GB: HttpBody<Data=Bytes> + Send + 'static, GB: Error + Send +'static, G::Future:Send + 'static, H: Service<Request<AxumBody>, Response = Response<HB>,Error = Infallible> + Clone + Send + 'static, HB:HttpBody<Data=Bytes> + Send + 'static, HB::Error: Error + Send + Sync +'static, H::Future:Send + 'static {
    type Response = AxumResponse;
    type Error = Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<AxumBody>) -> Self::Future {
      let is_grpc_req = is_grpc(&req);
        let grpc = self.grpc.clone();
        let http = self.http.clone();
        Box::pin(async move {
            if is_grpc_req {
                match tower::ServiceExt::oneshot(grpc,req).await {

                    Ok(res) => {
                        let (parts, body) = res.into_parts();
                        Ok(Response::from_parts(parts, to_axum_body(body))) },
                    Err(e)=> Ok(internal_error(e))
                }
            } else {
                match tower::ServiceExt::oneshot(http,req).await {
                    Ok(res) => {
                        let (parts, body) = res.into_parts();
                        Ok(Response::from_parts(parts, to_axum_body(body)))
                    },
                    Err(e)=> Ok(internal_error(e))
                }
            }
        })

    }
    // type
}
// Type alias for compatibility or external use
pub type TonicCompatible<G, H> = ContentTypeSwitch<G, H>;