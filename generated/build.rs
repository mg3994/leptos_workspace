use std::fs;
use std::path::Path;

fn main() {
    // 1. Ensure the output directory exists
    let proto_root = Path::new("../proto");
    let out_dir = Path::new("src/generated");
    if !Path::new(out_dir).exists() {
        fs::create_dir_all(out_dir).expect("Failed to create generated directory");
    }
    let protos = _collect_proto_files_as_strings(&proto_root);
    // 2. Run the codegen
    tonic_prost_build::configure()
        .build_server(true)
        // Build the client stubs
        .build_client(true)
        .out_dir(out_dir)
        // Build the server stubs

        .compile_protos(
            &protos,
            &[proto_root.to_string_lossy().to_string()],
        )
        .expect("protoc-rust-grpc failed");
    // 3️⃣ Generate mod.rs
    _generate_mod_rs(&out_dir);

    // 4️⃣ Generate lib.rs to re-export everything
    _generate_lib_rs(&out_dir);
    // 3. (Optional) Tell Cargo to rerun if proto files change
    println!("cargo:rerun-if-changed=../proto");

}


/// Recursively collect all `.proto` files
fn _collect_proto_files_as_strings(dir: &Path) -> Vec<String> {
    let mut protos = Vec::new();

    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            protos.extend(_collect_proto_files_as_strings(&path));
        } else if path.extension().and_then(|s| s.to_str()) == Some("proto") {
            protos.push(path.to_string_lossy().to_string()); // <-- String
        }
    }

    protos
}


// ---------------------------------------
// Generate mod.rs for the `generated` folder
fn _generate_mod_rs(dir: &Path) {
    let mut entries: Vec<_> = fs::read_dir(dir).unwrap().filter_map(Result::ok).collect();
    entries.sort_by_key(|e| e.path());

    let mut out = String::new();
    for e in entries {
        let path = e.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            if path.file_name().unwrap() == "mod.rs" {
                continue;
            }
            let name = path.file_stem().unwrap().to_str().unwrap();
            out.push_str(&format!("pub mod {};\n", name));
        }
    }

    fs::write(dir.join("mod.rs"), out).unwrap();
}

// ---------------------------------------
// Generate lib.rs to expose `generated` as root
fn _generate_lib_rs(_generated_dir: &Path) {
    // Optional: re-export functions for easier access
    let lib_content = "pub mod generated;\n\npub use generated::*;\n"; // there is no need of use related line , but just for reference
    fs::write("src/lib.rs", lib_content).unwrap();
}