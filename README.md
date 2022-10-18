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
```
The app takes 3 arguments.
1) the word to search for
2) the file in which to search relative to the place where it was invoked
3) an optional boolean parameter (true/false) if it should be case insensitive or not.

The case sensitivity check can be set either as the 3rd param or through a environment variable `IGNORE_CASE`. When set to `true` or `True` (case in sensitive so doesnt depend on capitalization), it will search without checking for case sensitivity/capitalization. The default is false. The argument gets priority over the env, so when setting both to different values, the argument one will be taken

Examples:
```bash
# Through arguments
$ minigrep frog file.txt true

# Through env
$ IGNORE_CASE=true minigrep frog file.txt

# Uses `true` in this case
$ IGNORE_CASE=false minigrep frog file.txt true

```
## Footer
This is a test project for myself to learn more in rust. This is no way, efficient and production grade. If you're looking for something similar, please use [ripgrep](https://github.com/BurntSushi/ripgrep) by Andrew Gallant.
