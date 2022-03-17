> NOTE: Latest working commit is [f961fc672a9f5f2bc3470c967ba3c49e2c3e03c5](https://github.com/Hugo4IT/Hug/tree/f961fc672a9f5f2bc3470c967ba3c49e2c3e03c5). I'm very busy at the moment so progress will be a little slow for a while.
> 
> Progress: I have decided to make a bootstrap compiler in python, then later make Hug self-hosted. You can follow progress here:
> 
> - [X] Command line arguments (`bootstrap.py`)
> - [X] Lexical Analysis (`bootstrap/lexer.py`)
> - [X] Syntax highlighting engine (`bootsrap/lexer.py`)
> - [X] Syntactic Analysis (`bootstrap/syntax.py`)
> - [X] Semantic Analysis (`bootstrap/ident.py`)
> - [ ] Compling to assembly (`bootstrap/compiler.py`)
> - [ ] Assembling to machine code using NASM or fasm (`bootstrap/linker.py`)
> - [ ] Linking with other libraries like `libc` (`bootstrap/linker.py`)

# hug

My own lil easy-to-use programming language inspired by GDScript & Rust, focused on being beginner-friendly while providing advanced features and incredible speed.

## Usage

> DISCLAIMER: Running is not possible yet, check progress on the compiler at the top of this README, or use the [latest working commit](https://github.com/Hugo4IT/Hug/tree/f961fc672a9f5f2bc3470c967ba3c49e2c3e03c5) (very old, and interpreted)

Minimum python version: `3.7` (due to `repr()`).

```log
python3.10 ./bootstrap.py --help
```
```
Usage:
  python3 ./bootstrap.py [options] <file>

Available options:
  --verbose,-v                Enable verbose output (same as --log-level INFO)
  --help,-h                   Print this help message
  --version,-V                Print the current version of bootstrap.py
  --log-level,-l <level>      Increase/decrease output verbosity (<level>: [error, warning, info, debug])
  --highlight-syntax,-H       Print a syntax highlighted version of your code
  --yes-to-all,-y             Automatically answer 'y' to all questions
```

You can run any of the examples provided in `examples/`