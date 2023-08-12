fn main() {
    cc::Build::new()
        .file("src/haptic.m")
        .compile("libhaptic.a");

    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=AppKit");
}
