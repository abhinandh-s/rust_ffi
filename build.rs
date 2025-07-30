use std::process::Command;

fn main() {
    // Compile the C library
    let output = Command::new("clang")
        .args(&[
            "-c",
            "hello.c",
            "-o",
            "hello.o"
        ])
        .output()
        .expect("Failed to compile C code");

    if !output.status.success() {
        panic!("C compilation failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Create static library
    let output = Command::new("ar")
        .args(&[
            "rcs",
            "libhello.a",
            "hello.o"
        ])
        .output()
        .expect("Failed to create static library");

    if !output.status.success() {
        panic!("Library creation failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Tell cargo to link the library
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=static=hello");
    
    // Rerun build script if C files change
    println!("cargo:rerun-if-changed=hello.c");
    println!("cargo:rerun-if-changed=hello.h");
}
