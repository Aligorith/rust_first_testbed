First test app to start learning Rust

Notes on Using Rust - Compiling/Running
=======================================
* Compile the code using 
  $ cargo build

* Compile & run the code using
  $ cargo run

* To enable backtrace support (useful for figuring out where the program crashed)
  set the env var before running the code via cargo

  $ set RUST_BACKTRACE=1

* This project was created by doing
  $ cargo new myapp --bin
  
  - "--bin " says that the project will be a standalone binary (not a library)
  - "cargo new" initialises the repo (including all the git stuff)