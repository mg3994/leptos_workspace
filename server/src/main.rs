mod grpc;
mod models;
mod api;

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use app::*;
use leptos::logging::log;

// pub mod generated{
//     pub mod echo {
//         tonic::include_proto!("echo"); // Ensure this matches your package name in .proto
//     }
// }
#[derive(Debug, Clone, Default)]
pub struct MyEcho;

#[tonic::async_trait] // <--- This is required
impl generated::echo::echo_server::Echo for MyEcho {
    async fn echo(
        &self,
        request: tonic::Request<generated::echo::EchoRequest>,
    ) -> Result<tonic::Response<generated::echo::EchoReply>, tonic::Status> {
        Ok(tonic::Response::new(generated::echo::EchoReply {
            message: format!("Echoing back: {}", request.get_ref().message),
        }))
    }
}


#[tokio::main]
async fn main() {
    // Load .env files.
    dotenvy::dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let axum_make_service = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Correct way to initialize the service
    let echo_impl = MyEcho::default();
    let echo_service = generated::echo::echo_server::EchoServer::new(echo_impl);

    let grpc_router = tonic::transport::Server::builder()
        .add_service(echo_service);
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, axum_make_service.into_make_service())
        .await
        .unwrap();
}


