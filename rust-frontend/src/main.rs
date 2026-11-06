//! Palet Vendéen Frontend - Dioxus Implementation
//!
//! The web application lives in the library crate (`lib.rs`), which wasm-pack
//! compiles to WASM. This binary is only a thin shim around that entry point.

fn main() {
    palet_frontend::start();
}
