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


# Guide Starts Here

```bash
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace> ui add avatar 
📦 Final set of resolved components: {"avatar"}
📦 Final set of cargo dependencies: {}
🔸 Project not initialized. Run 'ui init' to initialize the project first.
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace> cd app
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add avatar
📦 Final set of resolved components: {"avatar"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add badge
📦 Final set of resolved components: {"badge"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add bottom_nav
📦 Final set of resolved components: {"bottom_nav"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add breadcrumb
📦 Final set of resolved components: {"breadcrumb"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add button
📦 Final set of resolved components: {"button"}
📦 Final set of cargo dependencies: {}
Component button already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add button_action
📦 Final set of resolved components: {"button", "button_action"}
📦 Final set of cargo dependencies: {}
Component button already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add button_group
📦 Final set of resolved components: {"button_group"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add card
📦 Final set of resolved components: {"card"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add card_carousel
📦 Final set of resolved components: {"card_carousel"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add checkbox
📦 Final set of resolved components: {"checkbox"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add chips
📦 Final set of resolved components: {"chips"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add command
📦 Final set of resolved components: {"button", "command"}
📦 Final set of cargo dependencies: {}
📦 Final set of JS files: {"/hooks/lock_scroll.js"}
Component button already exists in mod.rs
  ✔️ JS files installed: [/hooks/lock_scroll.js]                                                                                                                                    
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add context_menu
📦 Final set of resolved components: {"context_menu"}
📦 Final set of cargo dependencies: {}
📦 Final set of JS files: {"/hooks/lock_scroll.js"}
  ✔️ JS files installed: [/hooks/lock_scroll.js]                                                                                                                                    
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add table
📦 Final set of resolved components: {"table"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add date_picker
📦 Final set of resolved components: {"date_picker"}
📦 Final set of cargo dependencies: {"time"}
  ✔️ Successfully added to Cargo.toml: [time] !                                                                                                                                     
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add dialog
📦 Final set of resolved components: {"dialog", "button"}
📦 Final set of cargo dependencies: {}
📦 Final set of JS files: {"/hooks/lock_scroll.js"}
Component dialog already exists in mod.rs
Component button already exists in mod.rs
  ✔️ JS files installed: [/hooks/lock_scroll.js]                                                                                                                                    
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add drag_and_drop
📦 Final set of resolved components: {"drag_and_drop"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add drawer
📦 Final set of resolved components: {"drawer"}
📦 Final set of cargo dependencies: {}
📦 Final set of JS files: {"/app/vaul_drawer.js"}
  ✔️ JS files installed: [/app/vaul_drawer.js]                                                                                                                                      
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add dropdown_menu
📦 Final set of resolved components: {"dropdown_menu"}
📦 Final set of cargo dependencies: {}
📦 Final set of JS files: {"/hooks/lock_scroll.js"}
  ✔️ JS files installed: [/hooks/lock_scroll.js]                                                                                                                                    
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add dropzone     
⚠️  Component 'dropzone' not found in registry. Skipping...
📦 Final set of resolved components: {}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add empty
📦 Final set of resolved components: {"empty"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add form
📦 Final set of resolved components: {"form", "label", "separator", "input"}
📦 Final set of cargo dependencies: {"serde", "validator", "strum"}
  ✔️ Successfully added to Cargo.toml: [serde, validator] !                                                                                                                         
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add input
📦 Final set of resolved components: {"input"}
📦 Final set of cargo dependencies: {}
Component input already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add input_group
📦 Final set of resolved components: {"input_group", "input", "textarea"}
📦 Final set of cargo dependencies: {}
Component input already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add input_otp
📦 Final set of resolved components: {"input_otp"}
📦 Final set of cargo dependencies: {}
📦 Final set of JS files: {"/components/otp.js"}
  ✔️ JS files installed: [/components/otp.js]                                                                                                                                       
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add input_phone
📦 Final set of resolved components: {"button", "input_phone", "input", "popover", "command"}
📦 Final set of cargo dependencies: {}
📦 Final set of JS files: {"/hooks/lock_scroll.js"}
Component button already exists in mod.rs
Component input already exists in mod.rs
Component command already exists in mod.rs
  ✔️ JS files installed: [/hooks/lock_scroll.js]                                                                                                                                    
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add item
📦 Final set of resolved components: {"item", "separator"}
📦 Final set of cargo dependencies: {}
Component separator already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add kbd
📦 Final set of resolved components: {"kbd"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add label
📦 Final set of resolved components: {"label"}
📦 Final set of cargo dependencies: {}
Component label already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add marquee
📦 Final set of resolved components: {"marquee", "mask"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add multi_select
📦 Final set of resolved components: {"multi_select"}
📦 Final set of cargo dependencies: {}
📦 Final set of JS files: {"/hooks/lock_scroll.js"}
  ✔️ JS files installed: [/hooks/lock_scroll.js]                                                                                                                                    
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add pagination
📦 Final set of resolved components: {"pagination", "button"}
📦 Final set of cargo dependencies: {"strum"}
Component button already exists in mod.rs
  All dependencies already exist in Cargo.toml                                                                                                                                      
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add popover
🔸 Registry request failed
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add popover
🔸 Registry request failed
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add popover
📦 Final set of resolved components: {"popover"}
📦 Final set of cargo dependencies: {}
Component popover already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add pressable
📦 Final set of resolved components: {"pressable"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add radio_button
📦 Final set of resolved components: {"radio_button"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add radio_button_group
📦 Final set of resolved components: {"radio_button_group"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add scroll_area
📦 Final set of resolved components: {"scroll_area"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add select
📦 Final set of resolved components: {"select"}
📦 Final set of cargo dependencies: {"strum"}
📦 Final set of JS files: {"/hooks/lock_scroll.js"}
Component select already exists in mod.rs
  All dependencies already exist in Cargo.toml                                                                                                                                        ✔️ JS files installed: [/hooks/lock_scroll.js]                                                                                                                                    
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add separator
📦 Final set of resolved components: {"separator"}
📦 Final set of cargo dependencies: {}
Component separator already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add sheet
📦 Final set of resolved components: {"sheet", "button"}
📦 Final set of cargo dependencies: {}
📦 Final set of JS files: {"/hooks/lock_scroll.js"}
Component button already exists in mod.rs
  ✔️ JS files installed: [/hooks/lock_scroll.js]                                                                                                                                    
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add skeleton
📦 Final set of resolved components: {"skeleton"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add slider
📦 Final set of resolved components: {"slider"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add sonner
📦 Final set of resolved components: {"sonner"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add spinner
📦 Final set of resolved components: {"spinner"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add status
📦 Final set of resolved components: {"status"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add switch
📦 Final set of resolved components: {"switch"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add table
📦 Final set of resolved components: {"table"}
📦 Final set of cargo dependencies: {}
Component table already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add tabs
📦 Final set of resolved components: {"tabs"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add textarea
📦 Final set of resolved components: {"textarea"}
📦 Final set of cargo dependencies: {}
Component textarea already exists in mod.rs
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add theme_toggle
📦 Final set of resolved components: {"theme_toggle"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add toast_custom 
⚠️  Component 'toast_custom' not found in registry. Skipping...
📦 Final set of resolved components: {}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> ui add tooltip
📦 Final set of resolved components: {"tooltip"}
📦 Final set of cargo dependencies: {}
PS C:\Users\manis\OneDrive\Desktop\app_server\leptos_workspace\app> 

```