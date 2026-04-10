#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i64), FloatLit(f64), StrLit(String),
    BoolLit(bool), NilLit, Ident(String),
    BinOp { left: Box<Expr>, op: BinOpKind, right: Box<Expr> },
    UnaryOp { op: UnaryOpKind, expr: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
    Index { obj: Box<Expr>, idx: Box<Expr> },
    Field { obj: Box<Expr>, field: String },
    ListLit(Vec<Expr>),
    MapLit(Vec<(Expr, Expr)>),
    Ternary { cond: Box<Expr>, then: Box<Expr>, or: Box<Expr> },
    NullSafe { expr: Box<Expr>, fallback: Box<Expr> },
    Pipe { left: Box<Expr>, right: Box<Expr> },
    Range { start: Box<Expr>, end: Box<Expr>, inclusive: bool },
}
#[derive(Debug, Clone)]
pub enum BinOpKind {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, NotEq, Gt, Lt, GtEq, LtEq,
    And, Or, BitAnd, BitOr, BitXor, Shl, Shr,
}
#[derive(Debug, Clone)]
pub enum UnaryOpKind { Neg, Not, Ref, Deref }
#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, type_ann: Option<String>, value: Option<Expr>, mutable: bool },
    Assign { name: String, value: Expr },
    If { cond: Expr, body: Vec<Stmt>, elif_branches: Vec<(Expr, Vec<Stmt>)>, else_body: Option<Vec<Stmt>> },
    While { cond: Expr, body: Vec<Stmt> },
    Loop { body: Vec<Stmt> },
    Each { var: String, iter: Expr, body: Vec<Stmt> },
    Match { expr: Expr, arms: Vec<(Expr, Vec<Stmt>)> },
    Fn { name: String, params: Vec<(String, String)>, return_type: Option<String>, body: Vec<Stmt>, is_async: bool },
    Class { name: String, body: Vec<Stmt> },
    Give(Option<Expr>), Break, Skip,
    ExprStmt(Expr),
    Use { path: Vec<String>, alias: Option<String> },
    TryCatch { body: Vec<Stmt>, catch_var: String, catch_body: Vec<Stmt> },
}
#[derive(Debug, Clone)]
pub struct Program { pub stmts: Vec<Stmt> }
