// This is a `main.rs` file, therefore `cargo` interprets this as the root of a binary target.

// TODO: fix this broken import. Create a new library target in the `src` directory.
//   The library target should expose a public function named `hello_world` that takes no arguments
//   and returns nothing.
use packages::hello_world;

// This is the entrypoint of the binary.

// A crate is the smallest unit of compilation in Rust. It's a tree of modules that produces
// either a library or an executable. There are two types, Binary crates and Library crates

// A packages is a bundle of one or more crates. It contains:
// - A Cargo.toml
// - At most one library crate
// - Any number of binary crates
// - At least one crate 

fn main() {
    hello_world();
}
