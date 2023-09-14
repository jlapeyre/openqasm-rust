#![allow(unused)]
// TODO: remove unused above

//    use std::fmt::Write;
//    use std::io::Read;

use std::fs;
use std::path::PathBuf;
use std::iter::FromIterator;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::Lexer;

use antlr_rust::token::{Token, TOKEN_EOF};
// gjl added CommonTokenFactory below
use antlr_rust::token_factory::{ArenaCommonFactory, OwningTokenFactory,CommonTokenFactory};
use antlr_rust::token_stream::{TokenStream, UnbufferedTokenStream};
use antlr_rust::tree::{
    ParseTree, ParseTreeListener, ParseTreeVisitor, ParseTreeVisitorCompat, ParseTreeWalker,
    TerminalNode, Tree, VisitChildren, Visitable,
};

use antlr_rust::InputStream;

use qasm3lexer::*;
use qasm3parserlistener::*;
use qasm3parser::qasm3Parser;

use crate::qasm3parser::{
    qasm3ParserContext, qasm3ParserContextType // , qasm3FileContext, HdrContext, RowContext,
};

use crate::qasm3parservisitor::qasm3ParserVisitor;

mod qasm3lexer;
mod qasm3parser;
mod qasm3parserlistener;
mod qasm3parservisitor;


pub fn lexer_test_qasm3(qasm_file_path: &PathBuf) {
    let qasm_program = fs::read_to_string(qasm_file_path)
        .expect("Unable to read file.");
    let mut _lexer = qasm3Lexer::new(InputStream::new(&*qasm_program));
            let mut string = String::new();
    {
        let mut token_source = UnbufferedTokenStream::new_unbuffered(&mut _lexer);
        while token_source.la(1) != TOKEN_EOF {
            {
                let token = token_source.lt(1).unwrap();

                let len = token.get_stop() as usize + 1 - token.get_start() as usize;
                string.extend(
                    format!(
                        "{},len {}:\n{}\n",
                        qasm3lexer::_SYMBOLIC_NAMES[token.get_token_type() as usize]
                            .unwrap_or(&format!("{}", token.get_token_type())),
                        len,
                        String::from_iter(
                            qasm_program.chars().skip(token.get_start() as usize).take(len)
                        )
                    )
                    .chars(),
                );
            }
            token_source.consume();
        }
    }
    println!("{}", string);
    println!(
        "{}",
        _lexer
            .get_interpreter()
            .unwrap()
            .get_dfa()
            .read()
            .to_lexer_string()
    );
}

struct MyListener {}

impl<'input> qasm3ParserListener<'input> for MyListener {}

impl<'input> ParseTreeListener<'input, qasm3ParserContextType> for MyListener {
    fn enter_every_rule(&mut self, ctx: &dyn qasm3ParserContext<'input>) {
        println!(
            "rule entered {}",
            qasm3parser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }
}

pub fn parser_test_qasm3(qasm_file_path: &PathBuf) {
//    println!("test started");
    // let tf = CommonTokenFactory::default();
    // let mut _lexer =
    //     qasm3Lexer::new_with_token_factory(InputStream::new("OPENQASM 3.0;\n".into()), &tf);

    let qasm_program = fs::read_to_string(qasm_file_path)
        .expect("Unable to read file.");
    let mut _lexer = qasm3Lexer::new(InputStream::new(&*qasm_program));

    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = qasm3Parser::new(token_source);
    parser.add_parse_listener(Box::new(MyListener {}));
    println!("\nstart parsing parser_test_qasm3");
    let result = parser.program();
    assert!(result.is_ok());
    dbg!(result);
    // assert_eq!(
    //     result.unwrap().to_string_tree(&*parser),
    //     "(csvFile (hdr (row (field V123) , (field V2) \\n)) (row (field d1) , (field d2) \\n))"
    // );
}
