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