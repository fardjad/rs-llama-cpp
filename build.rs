// Mostly based on https://github.com/tazz4843/whisper-rs/blob/master/sys/build.rs
extern crate bindgen;

use std::{env, path::PathBuf};

const LIB_NAME: &str = "rs-llama-cpp-wrapper";
const FUNCTION_NAME_PATTERN: &str = "rs_llama_cpp_.*";

fn main() {
    let target = env::var("TARGET").unwrap();

    // Link C++ standard library
    if let Some(cpp_stdlib) = get_cpp_link_stdlib(&target) {
        println!("cargo:rustc-link-lib=dylib={}", cpp_stdlib);
        println!("cargo:rustc-link-arg=-l{}", cpp_stdlib);
    }

    // Link macOS Accelerate framework for matrix calculations
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }

    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=llama");
    println!("cargo:rustc-link-lib=static={}", LIB_NAME);

    for file in &[
        "llama.cpp",
        "CMakeLists.txt",
        "rs-llama-cpp-wrapper.h",
        "rs-llama-cpp-wrapper.cpp",
        "run-inference.h",
        "run-inference.cpp",
    ] {
        println!("cargo:rerun-if-changed={}/{}", LIB_NAME, file);
    }

    let original_dir = env::current_dir().unwrap();

    // build lib
    env::set_current_dir(LIB_NAME)
        .unwrap_or_else(|_| panic!("Unable to change directory to {}", LIB_NAME));
    _ = std::fs::remove_dir_all("build");
    _ = std::fs::create_dir("build");
    env::set_current_dir("build")
        .unwrap_or_else(|_| panic!("Unable to change directory to {} build", LIB_NAME));

    let build_dir = env::current_dir().unwrap();

    env::set_var("CXXFLAGS", "-fPIC");
    env::set_var("CFLAGS", "-fPIC");

    let code = std::process::Command::new("cmake")
        .arg("..")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .status()
        .expect("Failed to generate build script");
    if code.code() != Some(0) {
        panic!("Failed to generate build script");
    }

    #[allow(clippy::suspicious_command_arg_space)]
    let code = std::process::Command::new("cmake")
        .arg("--build")
        .arg(".")
        .arg("--config Release")
        .status()
        .expect("Failed to build lib");
    if code.code() != Some(0) {
        panic!("Failed to build lib");
    }

    // move libllama.a to where Cargo expects it (OUT_DIR)
    #[cfg(target_os = "windows")]
    {
        std::fs::copy(
            "llama.cpp/Release/llama.lib",
            format!("{}/llama.lib", env::var("OUT_DIR").unwrap()),
        )
        .expect("Failed to copy lib");
        std::fs::copy(
            // TODO: change name
            format!("Release/{}.lib", LIB_NAME),
            format!("{}/{}.lib", env::var("OUT_DIR").unwrap(), LIB_NAME),
        )
        .expect("Failed to copy lib");
    }

    #[cfg(not(target_os = "windows"))]
    {
        std::fs::copy(
            "llama.cpp/libllama.a",
            format!("{}/libllama.a", env::var("OUT_DIR").unwrap()),
        )
        .expect("Failed to copy lib");
        std::fs::copy(
            format!("lib{}.a", LIB_NAME),
            format!("{}/lib{}.a", env::var("OUT_DIR").unwrap(), LIB_NAME),
        )
        .expect("Failed to copy lib");
    }

    env::set_current_dir(original_dir).expect("Unable to change directory to original dir");

    let bindings = bindgen::Builder::default()
        .header(format!("{}/{}.h", LIB_NAME, LIB_NAME).as_str())
        .allowlist_function(FUNCTION_NAME_PATTERN)
        .allowlist_recursively(true)
        .clang_args(&["-x", "c++"])
        .clang_arg(format!("-I./{}", LIB_NAME).as_str())
        .clang_arg(format!("-I./{}/llama.cpp", LIB_NAME).as_str())
        .clang_arg(format!("-I./{}/llama.cpp/examples", LIB_NAME).as_str())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate();

    match bindings {
        Ok(b) => {
            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            b.write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");
            b.write_to_file("src/bindings.rs")
                .expect("Couldn't write bindings!");
        }
        Err(e) => {
            println!("cargo:warning=Unable to generate bindings: {}", e);
            println!("cargo:warning=Using bundled bindings.rs, which may be out of date");
            // copy src/bindings.rs to OUT_DIR
            std::fs::copy(
                "src/bindings.rs",
                env::var("OUT_DIR").unwrap() + "/bindings.rs",
            )
            .expect("Unable to copy bindings.rs");
        }
    }

    // clean the modified files to prevent Cargo from complaining during crate publish
    _ = std::fs::remove_dir_all(build_dir);
}

// From https://github.com/alexcrichton/cc-rs/blob/fba7feded71ee4f63cfe885673ead6d7b4f2f454/src/lib.rs#L2462
fn get_cpp_link_stdlib(target: &str) -> Option<&'static str> {
    if target.contains("msvc") {
        None
    } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        Some("c++")
    } else if target.contains("android") {
        Some("c++_shared")
    } else {
        Some("stdc++")
    }
}
