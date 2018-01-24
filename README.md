# rust-tokenize

Tokenize a string (in the NLP sense) using a rust library and FFI.

## Getting Started

 Make sure you have [rust
 installed](https://www.rust-lang.org/en-US/install.html).

Use the provided Makefile to to build the dylib and link with a demo c program.

```
$ make

cargo build --release
   Compiling libc v0.2.36
   Compiling tokenize v0.1.0 (file:///Users/sam/src/tokenize)
    Finished release [optimized] target(s) in 0.72 secs
gcc -L"./target/release/" -ltokenize test.c -o tokenize_linked

```

Next running the example c program you can see the ffi function output
of the tokenizer:

```
$ ./tokenize_linked "Hello Hello Hello this is a test test test of tokens."

Hello this is a test of tokens

```
