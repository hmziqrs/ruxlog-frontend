use std::env;
use std::fs::File;
use std::io::Write;

use dotenvy::dotenv;

fn main() {
    println!("cargo:rerun-if-changed=.env");
    let dest_path = "./src/env.rs";
    let mut f = File::create(&dest_path).unwrap();

    // use the dotenv crate to get the .env values
    dotenv().ok();
    f.write_all(b"// This file is automatically generated by build.rs\n\n")
        .unwrap();
    for (key, value) in env::vars() {
        if key.starts_with("APP_") {
            let line = format!(
                "pub const {}: &'static str = \"{}\";\n",
                key,
                value.replace("\"", "\\\"")
            );
            f.write_all(line.as_bytes()).unwrap();
        }
    }
}

