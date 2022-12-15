use clap::Parser;
use std::path::PathBuf;
use openqasm;

#[derive(Parser)]
#[command(about = "Minimal demo/test of parsing and lexing QASM 3.0", long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("lex"))]// .to_string())]
    action: String,
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
