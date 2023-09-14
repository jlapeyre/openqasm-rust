# OpenQASM 3.0 Reference Parser for Rust

### Introduction

This package (crate-to-be) provides a parser for OpenQASM 3.0
in Rust. The parser is built from an [ANTLR](https://www.antlr.org/)
specification. This ANTLR specification is maintained in
the [grammar directory of the openqasm project](https://github.com/openqasm/openqasm/tree/main/source/grammar),
which also builds a Python target.

Rust is not supported in the ANTLR distribution.
Instead, this package depends on the [antlr4rust](https://github.com/rrevenantt/antlr4rust) crate, which
provides a Rust target for ANTLR.

### Build

Try `cargo build`. This builds a library and an exectuable for demonstration. This build
includes debugging information in the library and the executable.

Use `cargo build --release` to build a more performant version.

### Example

To run the example program via `cargo`, do this

```shell
cargo run --bin qasmparsedemo -- --action parse /path/to/your/qasm/program.qasm
```

or

```shell
cargo run --bin qasmparsedemo -- --action lex /path/to/your/qasm/program.qasm
```

You can also run the demo directly like this

```shell
./target/debug/qasmparsedemo --action parse /path/to/your/qasm/program.qasm
```

### Generating the library

The [lexer and parser](./src/) were built by adding the third-party rust target to the
ANTLR parser generator and using the commands
```shell
java -jar tool/target/antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -listener ./openqasm/qasm3Lexer.g4  ./openqasm/qasm3Parser.g4
java -jar tool/target/antlr4-4.8-2-SNAPSHOT-complete.jar -visitor -Dlanguage=Rust ./openqasm/qasm3Lexer.g4  ./openqasm/qasm3Parser.g4
```

#### TODO

I have not documented out to put together the ANTLR and the rust-target repos. That would be useful.

