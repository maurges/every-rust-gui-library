fn main() {
    let azul = std::env::var("AZUL_DIR_PATH").unwrap();
    println!("cargo:rustc-link-search={}", azul);
}
