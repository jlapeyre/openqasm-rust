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

Try `cargo build`. This builds a library and an exectuable for demonstration.


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
./target/debug/qasmparsedemo -- --action parse /path/to/your/qasm/program.qasm
```


