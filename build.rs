use std::env;

fn build() {
    let mut cfg = cc::Build::new();
    cfg.file("default_c/default.c");
    cfg.shared_flag(true);
    cfg.compile("def");
    println!("cargo:rerun-if-changed=default_c/default.c");
}

fn main() {
    build();
    // When cargo builds the main program, ld needs to know the shared library name and path.
    // So, tell cargo the information here.
    println!("cargo:rustc-link-search=native={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=def");
}
