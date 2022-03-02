> NOTE: Latest working commit is [f961fc672a9f5f2bc3470c967ba3c49e2c3e03c5](https://github.com/Hugo4IT/Hug/tree/f961fc672a9f5f2bc3470c967ba3c49e2c3e03c5). I'm very busy at the moment so progress will be a little slow for a while.

# hug

My own lil easy-to-use programming language inspired by GDScript & Rust, focused on being beginner-friendly while providing advanced features and incredible speed.

## Features

Currently working features are:

- External function calling (define dynamic library with `@extern(location = "path_to_lib.so") module name`, then a function with `@extern function name;`).
- Because of the prior feature, you can create Hug libraries in C or Rust. Tools are available for Rust libs.
- Very basic variables (no math yet, nor mutability)
- Lexer based off Rust's lexer
- Messy code that will need cleanup
- Interpreted by a VM (similair to how Java works)
- Bare bones core library with these variable types:
  - `Int8` - `Int128` - Sized integers
  - `UInt8` - `UInt128` - Sized unsigned integers (always positive)
  - `Float32`, `Float64` - Sized floats
  - `String` - Text
  - `Char` - A single character (8 bits)
  - `Function` - Pointer to Hug function
  - `ExternalFunction` - Pointer to external (dynamically loaded) function

## Usage

> **NOTE:** The `other/` directory contains a file for syntax highlighting in sublime text (not fully finished yet)

> **IMPORTANT:** Currently, the Core library is compiled to a dynamic library, the extension differs per platform. Make sure to change this line in `hug_core/hug/core.hug`:
> 
> ```hug
> @extern(location = "target/debug/libhug_core.dylib") module core
> ```
> 
> To any of these depending on your platform:
> 
> **Windows:**
> 
> ```hug
> @extern(location = "target/debug/libhug_core.dll") module core
> ```
> 
> **MacOS:**
> 
> ```hug
> @extern(location = "target/debug/libhug_core.dylib") module core
> ```
> 
> **Linux:**
> 
> ```hug
> @extern(location = "target/debug/libhug_core.so") module core
> ```

No precompiled binaries are available yet, but you can run it yourself using:

```bash
cargo run -- <args for hug>
```

For a list of commands run:

```bash
cargo run -- --help
```

Running a .hug file:

```bash
cargo run -- run <file>
```

Running the unit test:

```bash
cargo test
```

And lastly, disabling debug output:

```bash
cargo run --release -- <args for hug>
```
