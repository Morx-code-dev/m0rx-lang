#![allow(dead_code)]
use crate::parser::ast::*;
pub struct CodeGen { pub output: Vec<String> }
impl CodeGen {
    pub fn new() -> Self { CodeGen { output: Vec::new() } }
    fn emit(&mut self, line: &str) { self.output.push(line.to_string()); }
    fn ge(&self, e: &Expr) -> String {
        match e {
            Expr::IntLit(n) => n.to_string(),
            Expr::FloatLit(f) => f.to_string(),
            Expr::StrLit(s) => format!("\"{}\"", s),
            Expr::BoolLit(b) => b.to_string(),
            Expr::NilLit => "nil".to_string(),
            Expr::Ident(n) => n.clone(),
            Expr::Range { start, end, inclusive } => {
                if *inclusive { format!("{}...{}", self.ge(start), self.ge(end)) }
                else { format!("{}..{}", self.ge(start), self.ge(end)) }
            }
            Expr::BinOp { left, op, right } => {
                let o = match op {
                    BinOpKind::Add=>"+" ,BinOpKind::Sub=>"-",BinOpKind::Mul=>"*",
                    BinOpKind::Div=>"/",BinOpKind::Mod=>"%",BinOpKind::Pow=>"**",
                    BinOpKind::Eq=>"==",BinOpKind::NotEq=>"!=",BinOpKind::Gt=>">",
                    BinOpKind::Lt=>"<",BinOpKind::GtEq=>">=",BinOpKind::LtEq=>"<=",
                    BinOpKind::And=>"&&",BinOpKind::Or=>"||",BinOpKind::BitAnd=>"&",
                    BinOpKind::BitOr=>"|",BinOpKind::BitXor=>"^",BinOpKind::Shl=>"<<",BinOpKind::Shr=>">>",
                };
                format!("({} {} {})", self.ge(left), o, self.ge(right))
            }
            Expr::UnaryOp { op, expr } => {
                match op { UnaryOpKind::Neg => format!("(-{})", self.ge(expr)), _ => self.ge(expr) }
            }
            Expr::Call { name, args } => {
                let a: Vec<String> = args.iter().map(|a| self.ge(a)).collect();
                format!("{}({})", name, a.join(", "))
            }
            Expr::ListLit(items) => {
                let i: Vec<String> = items.iter().map(|x| self.ge(x)).collect();
                format!("[{}]", i.join(", "))
            }
            Expr::MapLit(pairs) => {
                let p: Vec<String> = pairs.iter().map(|(k,v)| format!("{}: {}", self.ge(k), self.ge(v))).collect();
                format!("{{{}}}", p.join(", "))
            }
            Expr::Index { obj, idx } => format!("{}[{}]", self.ge(obj), self.ge(idx)),
            Expr::Field { obj, field } => format!("{}.{}", self.ge(obj), field),
            Expr::NullSafe { expr, fallback } => format!("({} ?? {})", self.ge(expr), self.ge(fallback)),
            Expr::Pipe { left, right } => format!("({} |> {})", self.ge(left), self.ge(right)),
            Expr::Ternary { cond, then, or } => format!("({} ?: {} : {})", self.ge(cond), self.ge(then), self.ge(or)),
        }
    }
    fn gs(&mut self, s: &Stmt, i: usize) {
        let p = "    ".repeat(i);
        match s {
            Stmt::Let { name, type_ann, value, .. } => {
                let t = type_ann.as_deref().unwrap_or("any");
                let v = value.as_ref().map(|e| self.ge(e)).unwrap_or_else(|| "nil".to_string());
                self.emit(&format!("{}let {}: {} = {}", p, name, t, v));
            }
            Stmt::Assign { name, value } => { let v = self.ge(value); self.emit(&format!("{}{} = {}", p, name, v)); }
            Stmt::If { cond, body, elif_branches, else_body } => {
                let c = self.ge(cond); self.emit(&format!("{}if {} {{", p, c));
                for x in body { self.gs(x, i+1); } self.emit(&format!("{}}}", p));
                for (ec, eb) in elif_branches {
                    let ecs = self.ge(ec); self.emit(&format!("{}elif {} {{", p, ecs));
                    for x in eb { self.gs(x, i+1); } self.emit(&format!("{}}}", p));
                }
                if let Some(eb) = else_body {
                    self.emit(&format!("{}else {{", p));
                    for x in eb { self.gs(x, i+1); } self.emit(&format!("{}}}", p));
                }
            }
            Stmt::While { cond, body } => {
                let c = self.ge(cond); self.emit(&format!("{}while {} {{", p, c));
                for x in body { self.gs(x, i+1); } self.emit(&format!("{}}}", p));
            }
            Stmt::Loop { body } => {
                self.emit(&format!("{}loop {{", p));
                for x in body { self.gs(x, i+1); } self.emit(&format!("{}}}", p));
            }
            Stmt::Each { var, iter, body } => {
                let it = self.ge(iter); self.emit(&format!("{}each {} in {} {{", p, var, it));
                for x in body { self.gs(x, i+1); } self.emit(&format!("{}}}", p));
            }
            Stmt::Fn { name, params, return_type, body, is_async } => {
                let ps: Vec<String> = params.iter().map(|(n,t)| format!("{}: {}", n, t)).collect();
                let ret = return_type.as_deref().unwrap_or("nil");
                let ak = if *is_async { "async " } else { "" };
                self.emit(&format!("{}{}fn {}({}) -> {} {{", p, ak, name, ps.join(", "), ret));
                for x in body { self.gs(x, i+1); } self.emit(&format!("{}}}", p));
            }
            Stmt::Class { name, body } => {
                self.emit(&format!("{}class {} {{", p, name));
                for x in body { self.gs(x, i+1); } self.emit(&format!("{}}}", p));
            }
            Stmt::Give(expr) => {
                match expr {
                    Some(e) => { let v = self.ge(e); self.emit(&format!("{}give {}", p, v)); }
                    None => self.emit(&format!("{}give", p)),
                }
            }
            Stmt::Break => self.emit(&format!("{}break", p)),
            Stmt::Skip => self.emit(&format!("{}skip", p)),
            Stmt::Use { path, alias } => {
                let pt = path.join(".");
                match alias {
                    Some(a) => self.emit(&format!("{}use {} as {}", p, pt, a)),
                    None => self.emit(&format!("{}use {}", p, pt)),
                }
            }
            Stmt::TryCatch { body, catch_var, catch_body } => {
                self.emit(&format!("{}try {{", p));
                for x in body { self.gs(x, i+1); }
                self.emit(&format!("{}}} catch {} {{", p, catch_var));
                for x in catch_body { self.gs(x, i+1); }
                self.emit(&format!("{}}}", p));
            }
            Stmt::Match { expr, arms } => {
                let e = self.ge(expr); self.emit(&format!("{}match {} {{", p, e));
                for (pat, body) in arms {
                    let pt = self.ge(pat); self.emit(&format!("{}    {} => {{", p, pt));
                    for x in body { self.gs(x, i+2); }
                    self.emit(&format!("{}    }}", p));
                }
                self.emit(&format!("{}}}", p));
            }
            Stmt::ExprStmt(expr) => { let e = self.ge(expr); self.emit(&format!("{}{}", p, e)); }
        }
    }
    pub fn generate(&mut self, program: &Program) -> String {
        self.emit("// Generated by M0RX Compiler v0.1.0");
        self.emit("");
        for s in &program.stmts { self.gs(s, 0); }
        self.output.join("\n")
    }
}
