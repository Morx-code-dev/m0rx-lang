mod lexer;
mod parser;
mod analyzer;
mod codegen;

use lexer::Lexer;

fn main() {
    println!("M0RX Compiler v0.1.0");
    
    // Lexer test
    let code = r#"
        let name: str = "M0RX"
        let version: ant = 1
        showln("Hello M0RX!")
    "#;
    
    let mut lex = Lexer::new(code);
    let tokens = lex.tokenize();
    
    println!("Tokens found: {}", tokens.len());
    for tok in &tokens {
        println!("{:?}", tok);
    }
}
