// Versions of the core package and the compiler
// Used to ensure compatibility between core and a given plugin.
pub static CORE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &'static str = env!("RUSTC_VERSION");