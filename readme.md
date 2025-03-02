# Rust sandbox

Some rust tests. Or rest tusts.

## Doc

- https://doc.rust-lang.org/stable/book/
- https://doc.rust-lang.org/reference/introduction.html
- https://github.com/andyrbell/rust-for-java-developers/wiki/Lab-part-01
- https://stevedonovan.github.io/rust-gentle-intro/1-basics.html
- Cargo: https://doc.rust-lang.org/cargo/index.html

## Base commands

rustup doc

cargo new hello_world
cargo new my_lib --lib
cargo check
cargo build
cargo run
cargo build --release

update all dependencies:
cargo update

cargo test
cargo test --help

generate and open documentation of the project and its dependencies:
cargo doc --open
Doc with /// and //!

publish on crate.io
cargo login
cargo publish
cargo yank --vers 1.0.1
cargo yank --vers 1.0.1 --undo

install binary crates locally:
cargo install


## Crates

crate roots:
src/main.rs
src/lib.rs

src/bin/x.rs



https://crates.io/

Graph

- https://crates.io/crates/petgraph
- https://crates.io/crates/path-finding
- https://crates.io/crates/hierarchical_pathfinding

## Conventions

https://rust-lang.github.io/api-guidelines/


## Geo Rust

https://rust-gdal-cookbook.dend.ro/2_geometry
