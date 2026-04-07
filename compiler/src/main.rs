#![allow(dead_code)]
#![allow(unused_variables)]

mod lexer;
mod parser;
mod analyzer;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use analyzer::typechecker::TypeChecker;
use analyzer::semantic::SemanticAnalyzer;
use codegen::CodeGen;

fn run_file(path: &str) {
    let code = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("M0RX Error: Cannot read file '{}': {}", path, e);
            std::process::exit(1);
        }
    };
    compile_and_run(&code, path);
}

fn compile_and_run(code: &str, source: &str) {
    let mut lex = Lexer::new(code);
    let tokens = lex.tokenize();

    let mut parser = Parser::new(tokens);
    let program = parser.parse();

    let mut semantic = SemanticAnalyzer::new();
    semantic.analyze(&program);
    if !semantic.errors.is_empty() {
        for e in &semantic.errors {
            eprintln!("{}", e);
        }
        std::process::exit(1);
    }

    let mut checker = TypeChecker::new();
    let type_errors = checker.check(&program);
    if !type_errors.is_empty() {
        for e in &type_errors {
            eprintln!("{}", e);
        }
        std::process::exit(1);
    }

    let mut codegen = CodeGen::new();
    let output = codegen.generate(&program);

    // Execute: showln/show calls output करो
    execute(&program);
}

fn execute(program: &crate::parser::ast::Program) {
    use crate::parser::ast::{Stmt, Expr};

    let mut vars: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    fn eval_expr(expr: &Expr, vars: &std::collections::HashMap<String, String>) -> String {
        match expr {
            Expr::IntLit(n)   => n.to_string(),
            Expr::FloatLit(f) => f.to_string(),
            Expr::StrLit(s)   => s.clone(),
            Expr::BoolLit(b)  => b.to_string(),
            Expr::NilLit      => "nil".to_string(),
            Expr::Ident(name) => vars.get(name).cloned().unwrap_or_else(|| name.clone()),
            Expr::BinOp { left, op, right } => {
                let l = eval_expr(left, vars);
                let r = eval_expr(right, vars);
                let lf: f64 = l.parse().unwrap_or(0.0);
                let rf: f64 = r.parse().unwrap_or(0.0);
                use crate::parser::ast::BinOpKind::*;
                match op {
                    Add => {
                        if let (Ok(li), Ok(ri)) = (l.parse::<i64>(), r.parse::<i64>()) {
                            (li + ri).to_string()
                        } else { (lf + rf).to_string() }
                    }
                    Sub => (lf - rf).to_string(),
                    Mul => (lf * rf).to_string(),
                    Div => if rf != 0.0 { (lf / rf).to_string() } else { "0".to_string() },
                    Mod => ((lf as i64 % rf as i64)).to_string(),
                    Pow => lf.powf(rf).to_string(),
                    Eq    => (lf == rf || l == r).to_string(),
                    NotEq => (lf != rf).to_string(),
                    Gt    => (lf > rf).to_string(),
                    Lt    => (lf < rf).to_string(),
                    GtEq  => (lf >= rf).to_string(),
                    LtEq  => (lf <= rf).to_string(),
                    And   => ((l == "true") && (r == "true")).to_string(),
                    Or    => ((l == "true") || (r == "true")).to_string(),
                    _ => format!("({} op {})", l, r),
                }
            }
            Expr::Call { name, args } => {
                let eargs: Vec<String> = args.iter().map(|a| eval_expr(a, vars)).collect();
                match name.as_str() {
                    "showln" => {
                        println!("{}", eargs.join(" "));
                        String::new()
                    }
                    "show" => {
                        print!("{}", eargs.join(" "));
                        String::new()
                    }
                    "length" => eargs.first().map(|s| s.len().to_string()).unwrap_or("0".to_string()),
                    "upper"  => eargs.first().map(|s| s.to_uppercase()).unwrap_or_default(),
                    "lower"  => eargs.first().map(|s| s.to_lowercase()).unwrap_or_default(),
                    "trim"   => eargs.first().map(|s| s.trim().to_string()).unwrap_or_default(),
                    "absolute" => {
                        let n: f64 = eargs.first().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                        n.abs().to_string()
                    }
                    "root" => {
                        let n: f64 = eargs.first().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                        n.sqrt().to_string()
                    }
                    "power" => {
                        let base: f64 = eargs.first().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                        let exp: f64 = eargs.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                        base.powf(exp).to_string()
                    }
                    "ceil"  => { let n: f64 = eargs.first().and_then(|s| s.parse().ok()).unwrap_or(0.0); n.ceil().to_string() }
                    "floor" => { let n: f64 = eargs.first().and_then(|s| s.parse().ok()).unwrap_or(0.0); n.floor().to_string() }
                    "round" => { let n: f64 = eargs.first().and_then(|s| s.parse().ok()).unwrap_or(0.0); n.round().to_string() }
                    "glue"  => eargs.join(""),
                    _ => String::new(),
                }
            }
            Expr::ListLit(items) => {
                let vals: Vec<String> = items.iter().map(|i| eval_expr(i, vars)).collect();
                format!("[{}]", vals.join(", "))
            }
            _ => String::new(),
        }
    }

    fn exec_stmts(stmts: &[Stmt], vars: &mut std::collections::HashMap<String, String>) {
        for stmt in stmts {
            exec_stmt(stmt, vars);
        }
    }

    fn exec_stmt(stmt: &Stmt, vars: &mut std::collections::HashMap<String, String>) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let val = value.as_ref()
                    .map(|e| eval_expr(e, vars))
                    .unwrap_or_default();
                vars.insert(name.clone(), val);
            }
            Stmt::ExprStmt(expr) => {
                eval_expr(expr, vars);
            }
            Stmt::If { cond, body, elif_branches, else_body } => {
                let cv = eval_expr(cond, vars);
                let is_true = cv == "true" || (cv.parse::<f64>().unwrap_or(0.0) != 0.0);
                if is_true {
                    exec_stmts(body, vars);
                } else {
                    let mut done = false;
                    for (ec, eb) in elif_branches {
                        let ecv = eval_expr(ec, vars);
                        if ecv == "true" {
                            exec_stmts(eb, vars);
                            done = true;
                            break;
                        }
                    }
                    if !done {
                        if let Some(eb) = else_body {
                            exec_stmts(eb, vars);
                        }
                    }
                }
            }
            Stmt::While { cond, body } => {
                loop {
                    let cv = eval_expr(cond, vars);
                    if cv != "true" && cv.parse::<f64>().unwrap_or(0.0) == 0.0 { break; }
                    exec_stmts(body, vars);
                }
            }
            Stmt::Each { var, iter, body } => {
                let iter_val = eval_expr(iter, vars);
                // Simple range: parse [1, 2, 3] or DotDot range
                let items: Vec<String> = if iter_val.starts_with('[') {
                    iter_val.trim_matches(|c| c == '[' || c == ']')
                        .split(", ")
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                } else {
                    vec![iter_val]
                };
                for item in items {
                    vars.insert(var.clone(), item);
                    exec_stmts(body, vars);
                }
            }
            Stmt::Give(_) | Stmt::Break | Stmt::Skip => {}
            Stmt::Use { .. } => {}
            Stmt::Fn { .. } => {}
            Stmt::Class { .. } => {}
            _ => {}
        }
    }

    exec_stmts(&program.stmts, &mut vars);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 && args[1] == "run" {
        run_file(&args[2]);
        return;
    }

    if args.len() >= 2 && args[1] == "--version" {
        println!("M0RX Compiler v0.1.0");
        return;
    }

    // Default demo
    println!("M0RX Compiler v0.1.0");
    println!("Usage: morxc run <file.mrx>");
    println!("       morxc --version");
    println!("");
    println!("Example:");
    println!("  echo 'showln(\"Hello M0RX!\")' > hello.mrx");
    println!("  morxc run hello.mrx");
}
