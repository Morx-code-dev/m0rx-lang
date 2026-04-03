mod lexer;
mod parser;
mod analyzer;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use analyzer::typechecker::TypeChecker;
use analyzer::semantic::SemanticAnalyzer;

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

    // Lexer
    let mut lex = Lexer::new(code);
    let tokens = lex.tokenize();
    println!("Tokens:     {}", tokens.len());

    // Parser
    let mut parser = Parser::new(tokens);
    let program = parser.parse();
    println!("Statements: {}", program.stmts.len());

    // Semantic Analyzer
    let mut semantic = SemanticAnalyzer::new();
    semantic.analyze(&program);
    if semantic.errors.is_empty() {
        println!("Semantic:   PASSED");
    } else {
        for e in &semantic.errors {
            println!("{}", e);
        }
    }
    for w in &semantic.warnings {
        println!("{}", w);
    }

    // Type Checker
    let mut checker = TypeChecker::new();
    let type_errors = checker.check(&program);
    if type_errors.is_empty() {
        println!("Types:      PASSED");
    } else {
        for e in &type_errors {
            println!("{}", e);
        }
    }

    println!("====================");
    println!("M0RX: Build OK");
}
