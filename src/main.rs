fn main() {
    println!("{}", format!("{} - {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
}
