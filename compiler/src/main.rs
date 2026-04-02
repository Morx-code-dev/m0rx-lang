mod lexer;
mod parser;
mod analyzer;
mod codegen;

use lexer::Lexer;
use parser::Parser;

fn main() {
    println!("M0RX Compiler v0.1.0");

    let code = r#"
        let name: str = "M0RX"
        let version: ant = 1
        fn greet(name: str) {
            showln(name)
        }
        if version == 1 {
            showln("Welcome to M0RX!")
        }
    "#;

    let mut lex = Lexer::new(code);
    let tokens = lex.tokenize();
    println!("Tokens: {}", tokens.len());

    let mut parser = Parser::new(tokens);
    let program = parser.parse();
    println!("Statements parsed: {}", program.stmts.len());
    println!("Parser: OK");
}
