#![allow(dead_code)]
use std::collections::HashMap;
use crate::parser::ast::*;

#[derive(Debug, Clone)]
pub struct VarInfo {
    pub is_mutable: bool,
    pub is_used: bool,
    pub type_name: String,
}

pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, VarInfo>>,
    functions: HashMap<String, usize>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.last() {
            let unused: Vec<String> = scope
                .iter()
                .filter(|(n, i)| !i.is_used && !n.starts_with('_'))
                .map(|(n, _)| n.clone())
                .collect();
            for name in unused {
                self.warnings.push(format!(
                    "M0RX Warning: '{}' declared but never used", name
                ));
            }
        }
        self.scopes.pop();
    }

    fn define_var(&mut self, name: &str, mutable: bool, type_name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), VarInfo {
                is_mutable: mutable,
                is_used: false,
                type_name: type_name.to_string(),
            });
        }
    }

    fn use_var(&mut self, name: &str) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(info) = scope.get_mut(name) {
                info.is_used = true;
                return true;
            }
        }
        false
    }

    fn is_defined(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return true;
            }
        }
        false
    }

    fn is_builtin(name: &str) -> bool {
        matches!(name,
            "showln"|"show"|"ask"|"readline"|"emit"|"emitln"|
            "length"|"trim"|"upper"|"lower"|"absolute"|"root"|
            "append"|"remove"|"push"|"pop"|"assert"|"debug"|
            "logit"|"now"|"ceil"|"floor"|"round"|"fopen"|
            "fread"|"fwrite"|"fclose"|"quit"|"pause"|"flush"|
            "wipe"|"cut"|"glue"|"power"|"logn"|"pull"|"sift"|
            "morph"|"fold"|"arrange"|"flip"|"fcheck"|"fdrop"|
            "environ"|"argv"|"pid"|"expect"|"trace"|"benchmark"
        )
    }

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(name) => {
                if Self::is_builtin(name) { return; }
                if !self.use_var(name)
                    && !self.functions.contains_key(name)
                {
                    self.errors.push(format!(
                        "M0RX Error: '{}' used before declaration", name
                    ));
                }
            }
            Expr::BinOp { left, right, .. } => {
                self.check_expr(left);
                self.check_expr(right);
            }
            Expr::UnaryOp { expr, .. } => {
                self.check_expr(expr);
            }
            Expr::Call { name, args } => {
                if !Self::is_builtin(name)
                    && !self.functions.contains_key(name)
                    && !self.is_defined(name)
                {
                    self.warnings.push(format!(
                        "M0RX Warning: '{}' may not be defined", name
                    ));
                }
                for arg in args {
                    self.check_expr(arg);
                }
            }
            Expr::ListLit(items) => {
                for item in items { self.check_expr(item); }
            }
            Expr::MapLit(pairs) => {
                for (k, v) in pairs {
                    self.check_expr(k);
                    self.check_expr(v);
                }
            }
            Expr::Index { obj, idx } => {
                self.check_expr(obj);
                self.check_expr(idx);
            }
            Expr::Field { obj, .. } => {
                self.check_expr(obj);
            }
            Expr::NullSafe { expr, fallback } => {
                self.check_expr(expr);
                self.check_expr(fallback);
            }
            Expr::Pipe { left, right } => {
                self.check_expr(left);
                self.check_expr(right);
            }
            Expr::Ternary { cond, then, or } => {
                self.check_expr(cond);
                self.check_expr(then);
                self.check_expr(or);
            }
            _ => {}
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value, type_ann, .. } => {
                if let Some(val) = value {
                    self.check_expr(val);
                }
                let type_name = type_ann
                    .clone()
                    .unwrap_or_else(|| "any".to_string());
                self.define_var(name, true, &type_name);
            }
            Stmt::Assign { name, value } => {
                self.check_expr(value);
                if !self.is_defined(name) {
                    self.errors.push(format!(
                        "M0RX Error: '{}' is not defined", name
                    ));
                }
            }
            Stmt::If { cond, body, elif_branches, else_body } => {
                self.check_expr(cond);
                self.push_scope();
                for s in body { self.check_stmt(s); }
                self.pop_scope();
                for (ec, eb) in elif_branches {
                    self.check_expr(ec);
                    self.push_scope();
                    for s in eb { self.check_stmt(s); }
                    self.pop_scope();
                }
                if let Some(eb) = else_body {
                    self.push_scope();
                    for s in eb { self.check_stmt(s); }
                    self.pop_scope();
                }
            }
            Stmt::While { cond, body } => {
                self.check_expr(cond);
                self.push_scope();
                for s in body { self.check_stmt(s); }
                self.pop_scope();
            }
            Stmt::Loop { body } => {
                self.push_scope();
                for s in body { self.check_stmt(s); }
                self.pop_scope();
            }
            Stmt::Each { var, iter, body } => {
                self.check_expr(iter);
                self.push_scope();
                self.define_var(var, false, "any");
                for s in body { self.check_stmt(s); }
                self.pop_scope();
            }
            Stmt::Fn { name, params, body, .. } => {
                self.functions.insert(name.clone(), params.len());
                self.push_scope();
                for (pname, ptype) in params {
                    self.define_var(pname, true, ptype);
                }
                for s in body { self.check_stmt(s); }
                self.pop_scope();
            }
            Stmt::Class { name, body } => {
                self.functions.insert(name.clone(), 0);
                self.push_scope();
                for s in body { self.check_stmt(s); }
                self.pop_scope();
            }
            Stmt::TryCatch { body, catch_var, catch_body } => {
                self.push_scope();
                for s in body { self.check_stmt(s); }
                self.pop_scope();
                self.push_scope();
                self.define_var(catch_var, false, "str");
                for s in catch_body { self.check_stmt(s); }
                self.pop_scope();
            }
            Stmt::Match { expr, arms } => {
                self.check_expr(expr);
                for (pat, body) in arms {
                    self.check_expr(pat);
                    self.push_scope();
                    for s in body { self.check_stmt(s); }
                    self.pop_scope();
                }
            }
            Stmt::Give(expr) => {
                if let Some(e) = expr { self.check_expr(e); }
            }
            Stmt::ExprStmt(expr) => {
                self.check_expr(expr);
            }
            Stmt::Use { .. } | Stmt::Break | Stmt::Skip => {}
        }
    }

    pub fn analyze(&mut self, program: &Program) {
        let builtins = [
            "showln","show","ask","readline","emit","emitln",
            "length","trim","upper","lower","absolute","root",
            "append","remove","push","pop","assert","debug",
            "logit","now","ceil","floor","round","fopen",
            "fread","fwrite","fclose","quit","pause",
        ];
        for b in builtins {
            self.functions.insert(b.to_string(), 1);
        }
        for stmt in &program.stmts {
            self.check_stmt(stmt);
        }
    }
}
