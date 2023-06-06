fn main() {
    println!(
        "cargo:rustc-env=_ENV_TO_ARRAY_HEX=5bb32685b7e5bb32685b7ea2d07382db3744d43df89f97148eeae559ec4d0d1feefa2ee789da2d07382db3744d43df89f97148eeae559ec4d0d1feefa2ee789d",
    );
    println!("cargo:rustc-env=_ENV_TO_ARRAY_BS64=W7MmhbfqLQc4LbN0TUPfiflxSO6uVZ7E0NH+76LueJ0=",);
    println!("cargo:rustc-env=_ENV_TO_ARRAY_BS58=dwVAPpaonY26V6JH17ToUQ",);
    println!("cargo:rustc-env=_ENV_TO_ARRAY_BS32=Z0Z0Z0Z0");
    println!("cargo:rustc-env=_ENV_TO_ARRAY_BS85=VPRomVPRn");
}
