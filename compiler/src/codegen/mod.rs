use crate::parser::ast::*;

pub struct CodeGen {
    pub output: Vec<String>,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            output: Vec::new(),
        }
    }

    fn emit(&mut self, line: &str) {
        self.output.push(line.to_string());
    }

    fn gen_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::IntLit(n)   => n.to_string(),
            Expr::FloatLit(f) => f.to_string(),
            Expr::StrLit(s)   => format!("\"{}\"", s),
            Expr::BoolLit(b)  => b.to_string(),
            Expr::NilLit      => "nil".to_string(),
            Expr::Ident(name) => name.clone(),

            Expr::BinOp { left, op, right } => {
                let l = self.gen_expr(left);
                let r = self.gen_expr(right);
                let o = match op {
                    BinOpKind::Add   => "+",
                    BinOpKind::Sub   => "-",
                    BinOpKind::Mul   => "*",
                    BinOpKind::Div   => "/",
                    BinOpKind::Mod   => "%",
                    BinOpKind::Pow   => "**",
                    BinOpKind::Eq    => "==",
                    BinOpKind::NotEq => "!=",
                    BinOpKind::Gt    => ">",
                    BinOpKind::Lt    => "<",
                    BinOpKind::GtEq  => ">=",
                    BinOpKind::LtEq  => "<=",
                    BinOpKind::And   => "&&",
                    BinOpKind::Or    => "||",
                    BinOpKind::BitAnd => "&",
                    BinOpKind::BitOr  => "|",
                    BinOpKind::BitXor => "^",
                    BinOpKind::Shl    => "<<",
                    BinOpKind::Shr    => ">>",
                };
                format!("({} {} {})", l, o, r)
            }

            Expr::UnaryOp { op, expr } => {
                let e = self.gen_expr(expr);
                match op {
                    UnaryOpKind::Neg => format!("(-{})", e),
                    _                => format!("({})", e),
                }
            }

            Expr::Call { name, args } => {
                let a: Vec<String> = args
                    .iter()
                    .map(|a| self.gen_expr(a))
                    .collect();
                format!("{}({})", name, a.join(", "))
            }

            Expr::ListLit(items) => {
                let i: Vec<String> = items
                    .iter()
                    .map(|x| self.gen_expr(x))
                    .collect();
                format!("[{}]", i.join(", "))
            }

            Expr::MapLit(pairs) => {
                let p: Vec<String> = pairs
                    .iter()
                    .map(|(k, v)| format!(
                        "{}: {}",
                        self.gen_expr(k),
                        self.gen_expr(v)
                    ))
                    .collect();
                format!("{{{}}}", p.join(", "))
            }

            Expr::Index { obj, idx } => {
                format!("{}[{}]", self.gen_expr(obj), self.gen_expr(idx))
            }

            Expr::Field { obj, field } => {
                format!("{}.{}", self.gen_expr(obj), field)
            }

            Expr::NullSafe { expr, fallback } => {
                format!("({} ?? {})",
                    self.gen_expr(expr),
                    self.gen_expr(fallback)
                )
            }

            Expr::Pipe { left, right } => {
                format!("({} |> {})",
                    self.gen_expr(left),
                    self.gen_expr(right)
                )
            }

            Expr::Ternary { cond, then, or } => {
                format!("({} ?: {} : {})",
                    self.gen_expr(cond),
                    self.gen_expr(then),
                    self.gen_expr(or)
                )
            }
        }
    }

    fn gen_stmt(&mut self, stmt: &Stmt, indent: usize) {
        let pad = "    ".repeat(indent);
        match stmt {
            Stmt::Let { name, type_ann, value, .. } => {
                let t = type_ann.as_deref().unwrap_or("any");
                let v = value.as_ref()
                    .map(|e| self.gen_expr(e))
                    .unwrap_or_else(|| "nil".to_string());
                self.emit(&format!("{}let {}: {} = {}", pad, name, t, v));
            }

            Stmt::Assign { name, value } => {
                let v = self.gen_expr(value);
                self.emit(&format!("{}{} = {}", pad, name, v));
            }

            Stmt::If { cond, body, elif_branches, else_body } => {
                let c = self.gen_expr(cond);
                self.emit(&format!("{}if {} {{", pad, c));
                for s in body { self.gen_stmt(s, indent + 1); }
                self.emit(&format!("{}}}", pad));
                for (ec, eb) in elif_branches {
                    let ec_str = self.gen_expr(ec);
                    self.emit(&format!("{}elif {} {{", pad, ec_str));
                    for s in eb { self.gen_stmt(s, indent + 1); }
                    self.emit(&format!("{}}}", pad));
                }
                if let Some(eb) = else_body {
                    self.emit(&format!("{}else {{", pad));
                    for s in eb { self.gen_stmt(s, indent + 1); }
                    self.emit(&format!("{}}}", pad));
                }
            }

            Stmt::While { cond, body } => {
                let c = self.gen_expr(cond);
                self.emit(&format!("{}while {} {{", pad, c));
                for s in body { self.gen_stmt(s, indent + 1); }
                self.emit(&format!("{}}}", pad));
            }

            Stmt::Loop { body } => {
                self.emit(&format!("{}loop {{", pad));
                for s in body { self.gen_stmt(s, indent + 1); }
                self.emit(&format!("{}}}", pad));
            }

            Stmt::Each { var, iter, body } => {
                let i = self.gen_expr(iter);
                self.emit(&format!("{}each {} in {} {{", pad, var, i));
                for s in body { self.gen_stmt(s, indent + 1); }
                self.emit(&format!("{}}}", pad));
            }

            Stmt::Fn { name, params, return_type, body, is_async } => {
                let p: Vec<String> = params
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, t))
                    .collect();
                let ret = return_type.as_deref().unwrap_or("nil");
                let async_kw = if *is_async { "async " } else { "" };
                self.emit(&format!(
                    "{}{}fn {}({}) -> {} {{",
                    pad, async_kw, name, p.join(", "), ret
                ));
                for s in body { self.gen_stmt(s, indent + 1); }
                self.emit(&format!("{}}}", pad));
            }

            Stmt::Class { name, body } => {
                self.emit(&format!("{}class {} {{", pad, name));
                for s in body { self.gen_stmt(s, indent + 1); }
                self.emit(&format!("{}}}", pad));
            }

            Stmt::Give(expr) => {
                match expr {
                    Some(e) => {
                        let v = self.gen_expr(e);
                        self.emit(&format!("{}give {}", pad, v));
                    }
                    None => self.emit(&format!("{}give", pad)),
                }
            }

            Stmt::Break => {
                self.emit(&format!("{}break", pad));
            }

            Stmt::Skip => {
                self.emit(&format!("{}skip", pad));
            }

            Stmt::Use { path, alias } => {
                let p = path.join(".");
                match alias {
                    Some(a) => self.emit(&format!(
                        "{}use {} as {}", pad, p, a
                    )),
                    None => self.emit(&format!("{}use {}", pad, p)),
                }
            }

            Stmt::TryCatch { body, catch_var, catch_body } => {
                self.emit(&format!("{}try {{", pad));
                for s in body { self.gen_stmt(s, indent + 1); }
                self.emit(&format!("{}}} catch {} {{", pad, catch_var));
                for s in catch_body { self.gen_stmt(s, indent + 1); }
                self.emit(&format!("{}}}", pad));
            }

            Stmt::Match { expr, arms } => {
                let e = self.gen_expr(expr);
                self.emit(&format!("{}match {} {{", pad, e));
                for (pat, body) in arms {
                    let p = self.gen_expr(pat);
                    self.emit(&format!("{}    {} => {{", pad, p));
                    for s in body { self.gen_stmt(s, indent + 2); }
                    self.emit(&format!("{}    }}", pad));
                }
                self.emit(&format!("{}}}", pad));
            }

            Stmt::ExprStmt(expr) => {
                let e = self.gen_expr(expr);
                self.emit(&format!("{}{}", pad, e));
            }
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        self.emit("// Generated by M0RX Compiler v0.1.0");
        self.emit("");
        for stmt in &program.stmts {
            self.gen_stmt(stmt, 0);
        }
        self.output.join("\n")
    }
}
