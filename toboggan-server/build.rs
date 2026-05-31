#![allow(clippy::expect_used)]

use std::fs;
use std::path::Path;

fn main() {
    let web_dist_path = Path::new("../toboggan-web/dist");
    let index_html = web_dist_path.join("index.html");

    if index_html.exists() {
        println!("cargo:warning=web frontend dist folder found and valid");
        println!("cargo:rerun-if-changed=../toboggan-web/dist");
        if let Ok(entries) = fs::read_dir(web_dist_path) {
            for entry in entries.flatten() {
                println!("cargo:rerun-if-changed={}", entry.path().display());
            }
        }
    } else {
        println!("cargo:warning=web frontend dist not found — server will embed placeholder page");
        println!("cargo:warning=to build the full web UI:");
        println!("cargo:warning=  1. cd toboggan-web/toboggan-wasm && wasm-pack build --target web --release");
        println!("cargo:warning=  2. cd ../.. && npm install && npm run build");
        println!("cargo:warning=  3. cargo build -p toboggan-server (rebuild with embedded UI)");
        let dist = Path::new("../toboggan-web/dist");
        fs::create_dir_all(dist).expect("failed to create dist directory");
        fs::write(
            dist.join("index.html"),
            r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Toboggan</title></head><body><h1>Toboggan Server</h1><p>Web UI not built. Run <code>npm run build</code> in <code>toboggan-web/</code>.</p></body></html>"#,
        )
        .expect("failed to write placeholder index.html");
        println!("cargo:rerun-if-changed=build.rs");
    }
}
