
fn main() {
    println!("cargo:rustc-flags=-l dylib=stdc++");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .cpp(true)
        .file("src/byteutils.cpp")
        .file("src/CBuf.cpp")
        .file("src/picture.cpp")
        .file("src/woba.cpp")
        .compile("stackimport");
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/byteutils.cpp");
    println!("cargo:rerun-if-changed=src/CBuf.cpp");
    println!("cargo:rerun-if-changed=src/CStackFile.cpp");
    println!("cargo:rerun-if-changed=src/picture.cpp");
    println!("cargo:rerun-if-changed=src/woba.cpp");
    println!("cargo:rustc-link-lib=stackimport");
}
