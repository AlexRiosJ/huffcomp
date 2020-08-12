# huffcomp
[![Crate](https://img.shields.io/crates/v/huffcomp.svg)](https://crates.io/crates/huffcomp)

Huffman coding program for compression and decompression of text files.

## Installation

If you're a **Rust programmer**, huffcomp can be installed with `cargo`.

```
$ cargo install huffcomp
```

## Building

huffcomp can be build from source code using the Rust compiler.

```
$ git clone https://github.com/AlexRiosJ/huffcomp.git
$ cd huffcomp
$ cargo build --release
```

## Usage

It is possible for huffcomp to compress any kind of files that have UTF-8 valid encoding. (e.g. *.txt, *.c, *.rs, *.java, *.js)

To compress the file:

```
$ huffcomp -c <filename>
```

This command will generate a HUFF file (*.huff) which will have the same name as the original but with the huffcomp extension concatenated at the end.

To decompress a file it must have this extension and be compressed by huffcomp previously.

To decompress the file:

```
$ huffcomp -d <huffcomp_file>
```

### Contribute

To contribute, please fork the [repository](https://github.com/AlexRiosJ/huffcomp). If you find any bugs, issues or suggestions, please post your Issues and create your Pull Requests.

#### Used Techniques

- Huffman Coding Tree implementation.
- Bit level operations.
- Write and read files.
