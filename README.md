<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos --locked
```

Then run
```bash
cargo leptos new --git https://github.com/leptos-rs/start-axum-workspace/
```

to generate a new project template.

```bash
cd leptos_workspace
```

to go to your newly created project.
Feel free to explore the project structure, but the best place to start with your application code is in `app/src/lib.rs`.
Additionally, Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.

### Islands support

Note that for islands to work correctly, you need to have a `use app;` in your frontend `lib.rs` otherwise rustc / wasm_bindgen gets confused.
To prevent clippy from complaining, at the top of the `frontend/src/lib.rs` file place:
```rust
#[allow(clippy::single_component_path_imports)]
#[allow(unused_imports)]
use app;
```

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project

Cargo-leptos uses [Playwright](https://playwright.dev) as the end-to-end test tool.

Prior to the first run of the end-to-end tests run Playwright must be installed.
In the project's `end2end` directory run `npm install -D playwright @playwright/test` to install playwright and browser specific APIs.

To run the tests during development in the project root run:
```bash
cargo leptos end-to-end
```

To run tests for release in the project root run:
```bash
cargo leptos end-to-end --release
```
There are some examples tests are located in `end2end/tests` directory that pass tests with the sample Leptos app.

A web-based report on tests is available by running `npx playwright show-report` in the `end2end` directory.


## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
leptos_workspace
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="leptos_workspace"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.



# ReadMe Starts Here

### My Project Workspace

#### This repository demonstrates a modular Rust workspace architecture for building:

* Web applications with Leptos + Axum

* Mobile applications via gRPC (By MG: Idea Dropped)

* Postgres database integration (pgvector, PostGIS, and extensions)

* AI functionality via RIG crates

* Shared protobufs for cross-platform (Flutter / other frameworks) usage

# To Remind
Build Protobuf Rust Code
```
cargo build -p generated
```
#### Make Sure no version conflict
```bash
cargo tree -p sqlx
```
```rust
//! ```ignore
//! /// A middleware that does nothing, but just passes on the request.
//! async fn do_nothing_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, GrpcStatus> {
//!     Ok(next.run(req).await)
//! }
//!
//! /// A middleware that cancels the request with a grpc status-code
//! async fn cancel_request_middleware<B>(_req: Request<B>, _next: Next<B>) -> Result<Response, GrpcStatus> {
//!     Err(tonic::Status::cancelled("Canceled").into())
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     // Spawn the Server
//!     tokio::task::spawn(async move {
//!         // The first grpc-service has middleware that accepts the request.
//!         let grpc_router1 = Router::new()
//!             .nest_tonic(Test1Server::new(Test1Service))
//!             .layer(from_fn(do_nothing_middleware));
//!
//!         // The second grpc-service instead cancels the request
//!         let grpc_router2 = Router::new()
//!             .nest_tonic(Test2Server::new(Test2Service))
//!             .layer(from_fn(cancel_request_middleware));
//!
//!         // Merge both routers into one.
//!         let grpc_router = grpc_router1.merge(grpc_router2);
//!
//!         // This is the normal rest-router, to which all normal requests are routed
//!         let rest_router = Router::new()
//!             .nest("/", Router::new().route("/123", get(|| async move {})))
//!             .route("/", get(|| async move {}));
//!
//!         // Combine both services into one
//!         let service = RestGrpcService::new(rest_router, grpc_router).into_make_service();
//!
//!         // And serve at 127.0.0.1:8080
//!         let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
//!         axum::serve(listener, service)
//!             .await
//!             .unwrap();
//!     });
//!
//!     tokio::time::sleep(Duration::from_millis(100)).await;
//!
//!     // Connect to the server with a grpc-client
//!     let channel = Channel::from_static("http://127.0.0.1:8080")
//!         .connect()
//!         .await
//!         .unwrap();
//!
//!     let mut client1 = Test1Client::new(channel.clone());
//!     let mut client2 = Test2Client::new(channel);
//!
//!     // The first request will succeed
//!     client1.test1(Test1Request {}).await.unwrap();
//!
//!     // While the second one gives a grpc Status::Canceled code.
//!     assert_eq!(
//!         client2.test2(Test2Request {}).await.unwrap_err().code(),
//!         tonic::Code::Cancelled,
//!     );
//! }
//! ```

mod nest;
mod rest_grpc;
mod status;

pub use nest::NestTonic;
pub use rest_grpc::RestGrpcService;
pub use status::GrpcStatus;
```