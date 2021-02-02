use rustc_version::{version_meta, Channel};

fn main() {
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=nightly");
    }

    // only purpose is to expose a nightly cfg flag
    println!("cargo:rerun-if-changed=build.rs")
}
