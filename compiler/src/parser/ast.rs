#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    BoolLit(bool),
    NilLit,

    // Identifier
    Ident(String),

    // Binary Operation: left op right
    BinOp {
        left: Box<Expr>,
        op: BinOpKind,
        right: Box<Expr>,
    },

    // Unary Operation: op expr
    UnaryOp {
        op: UnaryOpKind,
        expr: Box<Expr>,
    },

    // Function Call: name(args)
    Call {
        name: String,
        args: Vec<Expr>,
    },

    // Index: list[0]
    Index {
        obj: Box<Expr>,
        idx: Box<Expr>,
    },

    // Field Access: obj.field
    Field {
        obj: Box<Expr>,
        field: String,
    },

    // List literal: [1, 2, 3]
    ListLit(Vec<Expr>),

    // Map literal: {key: val}
    MapLit(Vec<(Expr, Expr)>),

    // Ternary: cond ?: then : else
    Ternary {
        cond: Box<Expr>,
        then: Box<Expr>,
        or: Box<Expr>,
    },

    // Null safe: expr ?? fallback
    NullSafe {
        expr: Box<Expr>,
        fallback: Box<Expr>,
    },

    // Pipe: expr |> fn
    Pipe {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinOpKind {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, NotEq, Gt, Lt, GtEq, LtEq,
    And, Or,
    BitAnd, BitOr, BitXor, Shl, Shr,
}

#[derive(Debug, Clone)]
pub enum UnaryOpKind {
    Neg,   // -x
    Not,   // !!x
    Ref,   // &x
    Deref, // *x
}

#[derive(Debug, Clone)]
pub enum Stmt {
    // let name: type = value
    Let {
        name: String,
        type_ann: Option<String>,
        value: Option<Expr>,
        mutable: bool,
    },

    // name = value
    Assign {
        name: String,
        value: Expr,
    },

    // if cond { body } elif { } else { }
    If {
        cond: Expr,
        body: Vec<Stmt>,
        elif_branches: Vec<(Expr, Vec<Stmt>)>,
        else_body: Option<Vec<Stmt>>,
    },

    // while cond { body }
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },

    // loop { body }
    Loop {
        body: Vec<Stmt>,
    },

    // each item in list { body }
    Each {
        var: String,
        iter: Expr,
        body: Vec<Stmt>,
    },

    // match expr { case val => body }
    Match {
        expr: Expr,
        arms: Vec<(Expr, Vec<Stmt>)>,
    },

    // fn name(params) -> type { body }
    Fn {
        name: String,
        params: Vec<(String, String)>,
        return_type: Option<String>,
        body: Vec<Stmt>,
        is_async: bool,
    },

    // class name { body }
    Class {
        name: String,
        body: Vec<Stmt>,
    },

    // give expr (return)
    Give(Option<Expr>),

    // break
    Break,

    // skip (continue)
    Skip,

    // Expression statement
    ExprStmt(Expr),

    // use module
    Use {
        path: Vec<String>,
        alias: Option<String>,
    },

    // try { } catch e { }
    TryCatch {
        body: Vec<Stmt>,
        catch_var: String,
        catch_body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
