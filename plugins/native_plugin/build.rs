use copy_to_output::copy_to_output;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=plugin/*");
    copy_to_output("plugin", &env::var("PROFILE").unwrap()).expect("Could not copy");
}
