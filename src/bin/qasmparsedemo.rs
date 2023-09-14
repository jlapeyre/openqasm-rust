use clap::Parser;
use std::path::PathBuf;
use openqasm;

// TODO: Try to understand better the documentation and examples for Clap
// in order to make this example program better. Currently, most documentation
// for this example program is dumped into `long_about`.
// The problem is that while `clap`
// has a fair amount of example and tutorial material, it is very poor and
// nearly useless.

#[derive(Parser)]
#[command(about = "Minimal demo/test of parsing and lexing QASM 3.0",
          long_about = "
qasmparsedemo

Demo program for parsing OpenQASM 3 in Rust using the OpenQASM 3 reference
parser with a Rust target.

Example:

qasmparsedemo --action parse /path/to/example.qasm

",
)]
struct Cli {

    /// The action to perform, either `lex` or `parse`.
    #[arg(short, long, default_value_t = String::from("lex"))]// .to_string())]
    action: String,

    /// The path to the file to parse.
    #[arg(value_name = "QASM_FILE")]
    file: PathBuf
}

fn main() {
    let args  = Cli::parse();

    if args.action == "lex" {
        openqasm::lexer_test_qasm3(&args.file);
    }
    if args.action == "parse" {
        openqasm::parser_test_qasm3(&args.file);
    }
}
