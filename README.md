# MAKr

This programming language is not really made for actual use, just me experimenting with things.

## Example

This is an example featuring all current features (taken from `test/call-stack.prog`):

```hs
@prHelloWorld {
    push "Hello, World!"
    print
}

call @prHelloWorld
push "Second message"
print
call @prHelloWorld
push "Third message"
print
```

I wanted to add some comments to the code to explain what it does, but the language doesn't support that yet :/

## Usage

Running `makr --help` will show you all available options:

```ml
[hugo4it@hugo4it MAKr]$ bin/makr.release --help
```

> ```yaml
> Usage:
>   makr [options] <file> [even more options]
> Options:
>   -v,--verbose                       Verbose output, mostly useful for debugging
>   -h,--help                          Print this helpfully helpful helping help message
>   -s,--initial-stack-size <size>     Pre-allocate <size> bytes for the stack 
>   -S,--stack-expansion-step <size>   When the stack limit is reached, allocate <size> more bytes
> ```

### Installation

Clone the repo from github, then run `make release` in the root folder. This creates `bin/makr.release`.

### Setting up VSCode for "contributing"

Since I don't expect anyone to contribute, this is mostly for myself

Extensions (all available on OpenVSIX) with pre-configured settings (with `.vscode/*.json):

| Name     | Description                                                                                                          |
| :------- | :------------------------------------------------------------------------------------------------------------------- |
| clangd   | Does C and C++ completions, poorly. Run the `Generate compile_commands.json` task to configure. (requires [Bear][1]) |
| CodeLLDB | Debugging                                                                                                            |

  [1]: https://github.com/rizsotto/Bear