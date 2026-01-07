// idea Dropped
//src/grpc/mod.rs
use tonic::transport::Server;
use tonic::transport::server::Router;

pub mod echo;
/// Build all gRPC services and return a tonic Server builder
pub fn build_grpc_server() -> Router {
    // GRPC [Start]
    // Correct way to initialize the service
    let echo_impl = echo::MyEcho::default();
    let echo_service = generated::echo::echo_server::EchoServer::new(echo_impl);
    let mut grpc_builder = tonic::transport::Server::builder();

    grpc_builder.add_service(echo_service)


    // GRPC [End]
}