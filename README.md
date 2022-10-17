# Minigrep

This is a repository for the [I/O Project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) from the Rust Guide

## Usage
1. Clone this repo.
```bash
$ git clone https://github.com/Yakiyo/minigrep
```
2. Run with cargo
```bash
$ cargo run -- word-to-search path-to-file
```
or build with cargo, add the executable to your system path and invoke from the command line
```bash
$ cargo build --release
# then
$ minigrep word-to-search path-to-file
