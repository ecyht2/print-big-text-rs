# Print Big Text Rust
A rust library that prints a given text in it’s ascii-art form.

The C++ implementation of the library can be found [here](https://github.com/ecyht2/EEEE2065-cw1).

## Installation

### Dependency
The installation tutorial assumes that you have [rust](http://www.rust-lang.org) and it's package manager [cargo](https://doc.rust-lang.org/cargo/) installed on your working computer. If they aren't installed on your computer, you can follow the installation instructions on [rust website](https://www.rust-lang.org/tools/install). Cargo should be installed when you installed rust. Howerver, you can compiled it from source by following the instructions on [cargo documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Compiling Executable
This crate provides an executable binary to print text in ascii-art form. 

``` sh
git clone https://github.com/ecyht2/print-big-text-rs
cd print-big-text-rs
cargo build --release
```

The compiled executable would be located in `target/release/print-big-text-rs`

<!-- ### Using the Library -->
<!-- This crate has a library associated with it. To use the library add this in your `Cargo.toml` file. -->

<!-- ``` toml -->
<!-- ... -->

<!-- [dependencies] -->
<!-- ... -->
<!-- print_big_text_rs = "0.1.0" -->
<!-- ... -->
<!-- ``` -->

<!-- Alternatively it can be added via `cargo`. -->

<!-- ``` sh -->
<!-- cargo add print_big_text_rs -->
<!-- ``` -->

## Usage

The executable accepts text to be printed through arguments. 

Using `cargo run`

``` sh
git clone https://github.com/ecyht2/print-big-text-rs
cd print-big-text-rs
cargo run --release HI 123 By@
```

or if it is already compiled

``` sh
target/release/print-big-text-rs HI 123 By@
```

**Note:** The program will print a blank letter if the character isn't supported.

## Documentation
The API documentation of the library can be found in the project [GitHub pages](https://ecyht2.github.io/print-big-text-rs/)
