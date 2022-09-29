extern crate cc;

fn main() {
    cc::Build::new()
        .compiler("riscv64-cartesi-linux-gnu-cc")
        .flag("-mabi=lp64")
        .flag("-march=rv64ima")
        .file("src/rollup/bindings.c")
        .compile("libbindings.a");

    println!("cargo:rerun-if-changed=src/rollup/bindings.c,src/rollup/bindings.h");
    println!("cargo:rustc-link-lib=bindings");
}
