extern crate cc;

fn main() {
    cc::Build::new()
        // .compiler("/clang+llvm/bin/clang")
        .file("src/context.c")
        .file("../global_mem.c")
        .flag("-Wno-attributes")
        .compile("libcontext.a");

    println!("cargo:rerun-if-changed=src/context.c");
    println!("cargo:rerun-if-changed=../global_mem.c");
    println!("cargo:rustc-link-arg=-Wl,--export-dynamic");
}
