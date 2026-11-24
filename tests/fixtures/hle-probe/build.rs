fn main() {
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if target_env == "msvc" {
        println!("cargo:rustc-link-arg-cdylib=/DELAYLOAD:victory-probe-delayed.dll");
        println!("cargo:rustc-link-lib=delayimp");
    } else {
        println!("cargo:rustc-link-arg-cdylib=-Wl,--delayload=victory-probe-delayed.dll");
    }
}
