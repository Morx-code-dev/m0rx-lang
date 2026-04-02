mod lexer;
mod parser;
mod analyzer;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use analyzer::typechecker::TypeChecker;

fn main() {
    println!("M0RX Compiler v0.1.0");
    println!("====================");

    let code = r#"
        let name: str = "M0RX"
        let version: ant = 1
        let score: dbl = 9.5
        let active: bool = true

        fn greet(name: str) {
            showln(name)
        }

        if version == 1 {
            showln("Welcome to M0RX!")
        }
    "#;

    // Step 2: Lexer
    let mut lex = Lexer::new(code);
    let tokens = lex.tokenize();
    println!("Tokens found: {}", tokens.len());

    // Step 3: Parser
    let mut parser = Parser::new(tokens);
    let program = parser.parse();
    println!("Statements parsed: {}", program.stmts.len());

    // Step 4: Type Checker
    let mut checker = TypeChecker::new();
    let errors = checker.check(&program);
    if errors.is_empty() {
        println!("Type Check: PASSED");
    } else {
        for err in &errors {
            println!("{}", err);
        }
    }

    println!("====================");
    println!("M0RX: OK");
}
