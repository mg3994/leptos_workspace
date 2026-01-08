mod grpc;
mod models;
mod api;
mod axum_grpc;

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use app::*;
use leptos::logging::log;
use crate::axum_grpc::ContentTypeSwitch;

#[tokio::main]
async fn main() {
    // Load .env files.
    dotenvy::dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    //
    let grpc_server = grpc::build_grpc_server();

    //

    let axum_service = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);


    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    // 3. Combine them using the ContentTypeSwitch
    // The switch handles the routing based on "application/grpc"
    let combined_service = ContentTypeSwitch::new(grpc_server, axum_service);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    // axum::serve(listener, axum_service.into_make_service())
    //     .await
    //     .unwrap();
    axum::serve(listener,tower::make::Shared::new(combined_service))
        .await.unwrap();
}


