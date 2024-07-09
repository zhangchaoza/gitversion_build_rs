fn main() {
    println!("CARGO_PKG_VERSION       : {}", env!("CARGO_PKG_VERSION"));
    println!(
        "CARGO_PKG_VERSION_MAJOR : {}",
        env!("CARGO_PKG_VERSION_MAJOR")
    );
    println!(
        "CARGO_PKG_VERSION_MINOR : {}",
        env!("CARGO_PKG_VERSION_MINOR")
    );
    println!(
        "CARGO_PKG_VERSION_PATCH : {}",
        env!("CARGO_PKG_VERSION_PATCH")
    );
    println!(
        "CARGO_PKG_VERSION_PRE   : {}",
        env!("CARGO_PKG_VERSION_PRE")
    );
}
