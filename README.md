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
