#![allow(dead_code)]
#![allow(unused_variables)]
mod lexer; mod parser; mod analyzer; mod codegen;
use lexer::Lexer;
use parser::Parser;
use analyzer::typechecker::TypeChecker;
use analyzer::semantic::SemanticAnalyzer;

fn run_file(path: &str) {
    let code = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => { eprintln!("M0RX Error: Cannot read '{}': {}", path, e); std::process::exit(1); }
    };
    run_code(&code);
}

fn run_code(code: &str) {
    let mut lex = Lexer::new(code);
    let tokens = lex.tokenize();
    let mut p = Parser::new(tokens);
    let program = p.parse();
    let mut sem = SemanticAnalyzer::new();
    sem.analyze(&program);
    if !sem.errors.is_empty() { for e in &sem.errors { eprintln!("{}", e); } std::process::exit(1); }
    let mut chk = TypeChecker::new();
    let errs = chk.check(&program);
    if !errs.is_empty() { for e in &errs { eprintln!("{}", e); } std::process::exit(1); }
    execute(&program);
}

fn execute(program: &parser::ast::Program) {
    use parser::ast::{Stmt, Expr, BinOpKind, UnaryOpKind};
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    enum Val { Int(i64), Float(f64), Str(String), Bool(bool), List(Vec<Val>), Nil }

    impl Val {
        fn display(&self) -> String {
            match self {
                Val::Int(n) => n.to_string(),
                Val::Float(f) => if f.fract()==0.0 { (*f as i64).to_string() } else { f.to_string() },
                Val::Str(s) => s.clone(),
                Val::Bool(b) => b.to_string(),
                Val::Nil => "nil".to_string(),
                Val::List(l) => { let p:Vec<String>=l.iter().map(|v|v.display()).collect(); format!("[{}]",p.join(", ")) }
            }
        }
        fn to_f64(&self) -> f64 { match self { Val::Int(n)=>*n as f64, Val::Float(f)=>*f, Val::Str(s)=>s.parse().unwrap_or(0.0), Val::Bool(b)=>if *b {1.0} else {0.0}, _=>0.0 } }
        fn to_i64(&self) -> i64 { match self { Val::Int(n)=>*n, Val::Float(f)=>*f as i64, Val::Str(s)=>s.parse().unwrap_or(0), Val::Bool(b)=>if *b {1} else {0}, _=>0 } }
        fn truthy(&self) -> bool { match self { Val::Bool(b)=>*b, Val::Int(n)=>*n!=0, Val::Float(f)=>*f!=0.0, Val::Str(s)=>!s.is_empty(), Val::Nil=>false, Val::List(l)=>!l.is_empty() } }
    }

    struct Env { scopes: Vec<HashMap<String, Val>> }
    impl Env {
        fn new() -> Self { Env { scopes: vec![HashMap::new()] } }
        fn get(&self, name: &str) -> Val {
            for s in self.scopes.iter().rev() { if let Some(v) = s.get(name) { return v.clone(); } }
            Val::Nil
        }
        fn set(&mut self, name: &str, val: Val) {
            for s in self.scopes.iter_mut().rev() {
                if s.contains_key(name) { s.insert(name.to_string(), val); return; }
            }
            self.scopes.last_mut().unwrap().insert(name.to_string(), val);
        }
        fn def(&mut self, name: &str, val: Val) {
            self.scopes.last_mut().unwrap().insert(name.to_string(), val);
        }
        fn push(&mut self) { self.scopes.push(HashMap::new()); }
        fn pop(&mut self) { self.scopes.pop(); }
    }

    fn eval(expr: &Expr, env: &mut Env) -> Val {
        match expr {
            Expr::IntLit(n)   => Val::Int(*n),
            Expr::FloatLit(f) => Val::Float(*f),
            Expr::StrLit(s)   => Val::Str(s.clone()),
            Expr::BoolLit(b)  => Val::Bool(*b),
            Expr::NilLit      => Val::Nil,
            Expr::Ident(name) => env.get(name),
            Expr::Range { start, end, inclusive } => {
                let s = eval(start, env).to_i64();
                let e = eval(end, env).to_i64();
                let items: Vec<Val> = if *inclusive { (s..=e).map(Val::Int).collect() }
                                      else { (s..e).map(Val::Int).collect() };
                Val::List(items)
            }
            Expr::BinOp { left, op, right } => {
                let l = eval(left, env);
                let r = eval(right, env);
                match op {
                    BinOpKind::Add => match (&l,&r) {
                        (Val::Int(a),Val::Int(b)) => Val::Int(a+b),
                        (Val::Str(a),Val::Str(b)) => Val::Str(format!("{}{}",a,b)),
                        _ => Val::Float(l.to_f64()+r.to_f64()),
                    },
                    BinOpKind::Sub => match (&l,&r) { (Val::Int(a),Val::Int(b))=>Val::Int(a-b), _=>Val::Float(l.to_f64()-r.to_f64()) },
                    BinOpKind::Mul => match (&l,&r) { (Val::Int(a),Val::Int(b))=>Val::Int(a*b), _=>Val::Float(l.to_f64()*r.to_f64()) },
                    BinOpKind::Div => { let rf=r.to_f64(); if rf==0.0 {Val::Nil} else { let lf=l.to_f64(); if lf%rf==0.0 {Val::Int((lf/rf) as i64)} else {Val::Float(lf/rf)} } },
                    BinOpKind::Mod => Val::Int(l.to_i64() % r.to_i64()),
                    BinOpKind::Pow => Val::Float(l.to_f64().powf(r.to_f64())),
                    BinOpKind::Eq  => Val::Bool(l.display()==r.display() || (l.to_f64()==r.to_f64())),
                    BinOpKind::NotEq => Val::Bool(l.display()!=r.display()),
                    BinOpKind::Gt  => Val::Bool(l.to_f64()>r.to_f64()),
                    BinOpKind::Lt  => Val::Bool(l.to_f64()<r.to_f64()),
                    BinOpKind::GtEq => Val::Bool(l.to_f64()>=r.to_f64()),
                    BinOpKind::LtEq => Val::Bool(l.to_f64()<=r.to_f64()),
                    BinOpKind::And => Val::Bool(l.truthy() && r.truthy()),
                    BinOpKind::Or  => Val::Bool(l.truthy() || r.truthy()),
                    _ => Val::Nil,
                }
            }
            Expr::UnaryOp { op, expr } => {
                let v = eval(expr, env);
                match op { UnaryOpKind::Neg => match v { Val::Int(n)=>Val::Int(-n), Val::Float(f)=>Val::Float(-f), _=>Val::Nil }, _=>v }
            }
            Expr::Call { name, args } => {
                let eargs: Vec<Val> = args.iter().map(|a| eval(a, env)).collect();
                match name.as_str() {
                    "showln" => { let s:Vec<String>=eargs.iter().map(|v|v.display()).collect(); println!("{}",s.join(" ")); Val::Nil }
                    "show"   => { let s:Vec<String>=eargs.iter().map(|v|v.display()).collect(); print!("{}",s.join(" ")); Val::Nil }
                    "length" => match eargs.first() { Some(Val::Str(s))=>Val::Int(s.len() as i64), Some(Val::List(l))=>Val::Int(l.len() as i64), _=>Val::Int(0) },
                    "upper"  => if let Some(Val::Str(s))=eargs.first() { Val::Str(s.to_uppercase()) } else { Val::Nil },
                    "lower"  => if let Some(Val::Str(s))=eargs.first() { Val::Str(s.to_lowercase()) } else { Val::Nil },
                    "trim"   => if let Some(Val::Str(s))=eargs.first() { Val::Str(s.trim().to_string()) } else { Val::Nil },
                    "absolute" => Val::Float(eargs.first().map(|v|v.to_f64().abs()).unwrap_or(0.0)),
                    "root"   => Val::Float(eargs.first().map(|v|v.to_f64().sqrt()).unwrap_or(0.0)),
                    "power"  => { let b=eargs.first().map(|v|v.to_f64()).unwrap_or(0.0); let e=eargs.get(1).map(|v|v.to_f64()).unwrap_or(0.0); Val::Float(b.powf(e)) },
                    "ceil"   => Val::Int(eargs.first().map(|v|v.to_f64().ceil() as i64).unwrap_or(0)),
                    "floor"  => Val::Int(eargs.first().map(|v|v.to_f64().floor() as i64).unwrap_or(0)),
                    "round"  => Val::Int(eargs.first().map(|v|v.to_f64().round() as i64).unwrap_or(0)),
                    "glue"   => { let p:Vec<String>=eargs.iter().map(|v|v.display()).collect(); Val::Str(p.join("")) },
                    "asStr"  => Val::Str(eargs.first().map(|v|v.display()).unwrap_or_default()),
                    "asInt"  => Val::Int(eargs.first().map(|v|v.to_i64()).unwrap_or(0)),
                    "asFloat"=> Val::Float(eargs.first().map(|v|v.to_f64()).unwrap_or(0.0)),
                    "has"    => if let (Some(Val::Str(s)),Some(Val::Str(p)))=(eargs.first(),eargs.get(1)) { Val::Bool(s.contains(p.as_str())) } else { Val::Bool(false) },
                    "quit"   => std::process::exit(0),
                    _ => Val::Nil,
                }
            }
            Expr::ListLit(items) => Val::List(items.iter().map(|i| eval(i, env)).collect()),
            _ => Val::Nil,
        }
    }

    fn exec(stmts: &[Stmt], env: &mut Env) -> bool {
        for s in stmts { if run_stmt(s, env) { return true; } }
        false
    }

    fn run_stmt(stmt: &Stmt, env: &mut Env) -> bool {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let v = value.as_ref().map(|e| eval(e, env)).unwrap_or(Val::Nil);
                env.def(name, v); false
            }
            Stmt::Assign { name, value } => {
                let v = eval(value, env);
                env.set(name, v); false
            }
            Stmt::ExprStmt(expr) => { eval(expr, env); false }
            Stmt::If { cond, body, elif_branches, else_body } => {
                if eval(cond, env).truthy() {
                    env.push(); let b=exec(body, env); env.pop(); return b;
                }
                for (ec, eb) in elif_branches {
                    if eval(ec, env).truthy() {
                        env.push(); let b=exec(eb, env); env.pop(); return b;
                    }
                }
                if let Some(eb) = else_body { env.push(); let b=exec(eb, env); env.pop(); return b; }
                false
            }
            Stmt::While { cond, body } => {
                loop {
                    if !eval(cond, env).truthy() { break; }
                    env.push(); let brk=exec(body, env); env.pop();
                    if brk { break; }
                }
                false
            }
            Stmt::Loop { body } => {
                loop { env.push(); let brk=exec(body, env); env.pop(); if brk { break; } }
                false
            }
            Stmt::Each { var, iter, body } => {
                let items = match eval(iter, env) {
                    Val::List(l) => l,
                    other => vec![other],
                };
                for item in items {
                    env.set(var, item);
                    env.push(); let brk=exec(body, env); env.pop();
                    if brk { break; }
                }
                false
            }
            Stmt::Break => true,
            Stmt::Give(_) | Stmt::Skip => false,
            Stmt::Use { .. } | Stmt::Fn { .. } | Stmt::Class { .. } => false,
            _ => false,
        }
    }

    let mut env = Env::new();
    exec(&program.stmts, &mut env);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "run" { run_file(&args[2]); return; }
    if args.len() >= 2 && args[1] == "--version" { println!("M0RX Compiler v0.1.0"); return; }
    println!("M0RX Compiler v0.1.0");
    println!("Usage: morxc run <file.mrx>");
}
