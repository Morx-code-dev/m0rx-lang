use std::collections::HashMap;
use super::types::Type;
use crate::parser::ast::*;

pub struct TypeChecker {
    // Variable name → Type
    scope: Vec<HashMap<String, Type>>,
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            scope: vec![HashMap::new()],
            errors: Vec::new(),
        }
    }

    // Error add करो
    fn error(&mut self, msg: &str) {
        self.errors.push(format!("M0RX Type Error: {}", msg));
    }

    // New scope push करो
    fn push_scope(&mut self) {
        self.scope.push(HashMap::new());
    }

    // Scope pop करो
    fn pop_scope(&mut self) {
        self.scope.pop();
    }

    // Variable define करो
    fn define(&mut self, name: &str, ty: Type) {
        if let Some(scope) = self.scope.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    // Variable lookup करो
    fn lookup(&self, name: &str) -> Option<&Type> {
        for scope in self.scope.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty);
            }
        }
        None
    }

    // Expression का type निकालो
    pub fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::IntLit(_)   => Type::Ant,
            Expr::FloatLit(_) => Type::Dbl,
            Expr::StrLit(_)   => Type::Str,
            Expr::BoolLit(_)  => Type::Bool,
            Expr::NilLit      => Type::Nil,

            Expr::Ident(name) => {
                if let Some(ty) = self.lookup(name) {
                    ty.clone()
                } else {
                    self.error(&format!("'{}' is not defined", name));
                    Type::Unknown
                }
            }

            Expr::BinOp { left, op, right } => {
                let lt = self.check_expr(left);
                let rt = self.check_expr(right);
                match op {
                    BinOpKind::Add | BinOpKind::Sub |
                    BinOpKind::Mul | BinOpKind::Div |
                    BinOpKind::Mod | BinOpKind::Pow => {
                        if !lt.is_compatible(&rt) {
                            self.error(&format!(
                                "Type mismatch: {} and {}",
                                lt.to_str(), rt.to_str()
                            ));
                        }
                        lt
                    }
                    BinOpKind::Eq | BinOpKind::NotEq |
                    BinOpKind::Gt | BinOpKind::Lt |
                    BinOpKind::GtEq | BinOpKind::LtEq => {
                        Type::Bool
                    }
                    BinOpKind::And | BinOpKind::Or => {
                        Type::Bool
                    }
                    _ => lt,
                }
            }

            Expr::UnaryOp { expr, .. } => {
                self.check_expr(expr)
            }

            Expr::Call { name, args } => {
                // Built-in functions check
                match name.as_str() {
                    "showln" | "show" | "emit" | "emitln" => {
                        for arg in args {
                            self.check_expr(arg);
                        }
                        Type::Nil
                    }
                    "ask" | "readline" => Type::Str,
                    "length" => Type::Ant,
                    "absolute" | "root" | "ceil" |
                    "floor" | "round" => Type::Dbl,
                    _ => {
                        for arg in args {
                            self.check_expr(arg);
                        }
                        Type::Any
                    }
                }
            }

            Expr::ListLit(items) => {
                let inner = if items.is_empty() {
                    Type::Any
                } else {
                    self.check_expr(&items[0])
                };
                for item in items.iter().skip(1) {
                    self.check_expr(item);
                }
                Type::List(Box::new(inner))
            }

            Expr::MapLit(pairs) => {
                for (k, v) in pairs {
                    self.check_expr(k);
                    self.check_expr(v);
                }
                Type::Map(Box::new(Type::Any), Box::new(Type::Any))
            }

            Expr::Field { obj, .. } => {
                self.check_expr(obj);
                Type::Any
            }

            Expr::Index { obj, idx } => {
                self.check_expr(obj);
                self.check_expr(idx);
                Type::Any
            }

            Expr::NullSafe { expr, fallback } => {
                let t = self.check_expr(expr);
                self.check_expr(fallback);
                t
            }

            Expr::Pipe { left, right } => {
                self.check_expr(left);
                self.check_expr(right)
            }

            Expr::Ternary { cond, then, or } => {
                self.check_expr(cond);
                let t = self.check_expr(then);
                self.check_expr(or);
                t
            }
        }
    }

    // Statement check करो
    pub fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, type_ann, value, .. } => {
                let val_type = if let Some(expr) = value {
                    self.check_expr(expr)
                } else {
                    Type::Nil
                };
                let declared_type = if let Some(ann) = type_ann {
                    Type::from_str(ann)
                } else {
                    val_type.clone()
                };
                if !declared_type.is_compatible(&val_type) {
                    self.error(&format!(
                        "Variable '{}': declared as '{}' but got '{}'",
                        name,
                        declared_type.to_str(),
                        val_type.to_str()
                    ));
                }
                self.define(name, declared_type);
            }

            Stmt::Assign { name, value } => {
                let val_type = self.check_expr(value);
                if let Some(existing) = self.lookup(name).cloned() {
                    if !existing.is_compatible(&val_type) {
                        self.error(&format!(
                            "Cannot assign '{}' to variable '{}' of type '{}'",
                            val_type.to_str(), name, existing.to_str()
                        ));
                    }
                } else {
                    self.error(&format!("'{}' is not defined", name));
                }
            }

            Stmt::If { cond, body, elif_branches, else_body } => {
                let ct = self.check_expr(cond);
                if ct != Type::Bool && ct != Type::Any {
                    self.error("if condition must be bool");
                }
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
                self.define(var, Type::Any);
                for s in body { self.check_stmt(s); }
                self.pop_scope();
            }

            Stmt::Fn { name, params, return_type, body, .. } => {
                let ret = return_type
                    .as_deref()
                    .map(Type::from_str)
                    .unwrap_or(Type::Nil);
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|(_, t)| Type::from_str(t))
                    .collect();
                self.define(name, Type::Fn {
                    params: param_types,
                    ret: Box::new(ret),
                });
                self.push_scope();
                for (pname, ptype) in params {
                    self.define(pname, Type::from_str(ptype));
                }
                for s in body { self.check_stmt(s); }
                self.pop_scope();
            }

            Stmt::Give(expr) => {
                if let Some(e) = expr {
                    self.check_expr(e);
                }
            }

            Stmt::ExprStmt(expr) => {
                self.check_expr(expr);
            }

            Stmt::Use { .. } => {}
            Stmt::Break => {}
            Stmt::Skip => {}

            Stmt::TryCatch { body, catch_var, catch_body } => {
                self.push_scope();
                for s in body { self.check_stmt(s); }
                self.pop_scope();
                self.push_scope();
                self.define(catch_var, Type::Str);
                for s in catch_body { self.check_stmt(s); }
                self.pop_scope();
            }

            Stmt::Class { name, body } => {
                self.define(name, Type::Any);
                self.push_scope();
                for s in body { self.check_stmt(s); }
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
        }
    }

    // पूरा program check करो
    pub fn check(&mut self, program: &Program) -> Vec<String> {
        for stmt in &program.stmts {
            self.check_stmt(stmt);
        }
        self.errors.clone()
    }
}
