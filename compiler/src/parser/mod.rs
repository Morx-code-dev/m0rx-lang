pub mod ast;
use ast::*;
use crate::lexer::token::Token;
pub struct Parser { tokens: Vec<Token>, pos: usize }
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Parser { tokens, pos: 0 } }
    fn cur(&self) -> &Token { self.tokens.get(self.pos).unwrap_or(&Token::EOF) }
    fn adv(&mut self) -> &Token { let t = self.tokens.get(self.pos).unwrap_or(&Token::EOF); self.pos += 1; t }
    fn skip_nl(&mut self) { while self.cur() == &Token::Newline { self.adv(); } }
    fn expect(&mut self, t: &Token) -> bool {
        if self.cur() == t { self.adv(); true }
        else { eprintln!("M0RX Parse Error: expected {:?}, got {:?}", t, self.cur()); false }
    }
    fn parse_type(&mut self) -> String {
        match self.cur().clone() {
            Token::Tiny=>{self.adv();"tiny".into()} Token::Short=>{self.adv();"short".into()}
            Token::Ant=>{self.adv();"ant".into()} Token::Long=>{self.adv();"long".into()}
            Token::Vast=>{self.adv();"vast".into()} Token::Utiny=>{self.adv();"utiny".into()}
            Token::Ushort=>{self.adv();"ushort".into()} Token::Uant=>{self.adv();"uant".into()}
            Token::Ulong=>{self.adv();"ulong".into()} Token::Half=>{self.adv();"half".into()}
            Token::Dbl=>{self.adv();"dbl".into()} Token::Precise=>{self.adv();"precise".into()}
            Token::Chr=>{self.adv();"chr".into()} Token::Str=>{self.adv();"str".into()}
            Token::Txt=>{self.adv();"txt".into()} Token::Bool=>{self.adv();"bool".into()}
            Token::Nil=>{self.adv();"nil".into()} Token::List=>{self.adv();"list".into()}
            Token::Map=>{self.adv();"map".into()} Token::Set=>{self.adv();"set".into()}
            Token::Blob=>{self.adv();"blob".into()} Token::Tensor=>{self.adv();"tensor".into()}
            Token::Void=>{self.adv();"nil".into()}
            Token::Identifier(s)=>{let v=s.clone();self.adv();v}
            _ => "any".into()
        }
    }
    fn parse_primary(&mut self) -> Expr {
        match self.cur().clone() {
            Token::IntLiteral(n) => { let v=n; self.adv(); Expr::IntLit(v) }
            Token::FloatLiteral(f) => { let v=f; self.adv(); Expr::FloatLit(v) }
            Token::StringLiteral(s) => { let v=s.clone(); self.adv(); Expr::StrLit(v) }
            Token::True => { self.adv(); Expr::BoolLit(true) }
            Token::False => { self.adv(); Expr::BoolLit(false) }
            Token::Null | Token::Nil => { self.adv(); Expr::NilLit }
            Token::Identifier(name) => {
                let n = name.clone(); self.adv();
                if self.cur() == &Token::LParen {
                    self.adv();
                    let mut args = Vec::new();
                    while self.cur() != &Token::RParen && self.cur() != &Token::EOF {
                        self.skip_nl();
                        if self.cur() == &Token::RParen { break; }
                        args.push(self.parse_expr());
                        if self.cur() == &Token::Comma { self.adv(); }
                    }
                    self.expect(&Token::RParen);
                    Expr::Call { name: n, args }
                } else if self.cur() == &Token::Dot {
                    self.adv();
                    if let Token::Identifier(f) = self.cur().clone() {
                        let field = f.clone(); self.adv();
                        Expr::Field { obj: Box::new(Expr::Ident(n)), field }
                    } else { Expr::Ident(n) }
                } else if self.cur() == &Token::LBracket {
                    self.adv();
                    let idx = self.parse_expr();
                    self.expect(&Token::RBracket);
                    Expr::Index { obj: Box::new(Expr::Ident(n)), idx: Box::new(idx) }
                } else { Expr::Ident(n) }
            }
            Token::LParen => { self.adv(); let e=self.parse_expr(); self.expect(&Token::RParen); e }
            Token::LBracket => {
                self.adv();
                let mut items = Vec::new();
                while self.cur() != &Token::RBracket && self.cur() != &Token::EOF {
                    items.push(self.parse_expr());
                    if self.cur() == &Token::Comma { self.adv(); }
                }
                self.expect(&Token::RBracket); Expr::ListLit(items)
            }
            Token::LBrace => {
                self.adv();
                let mut pairs = Vec::new();
                while self.cur() != &Token::RBrace && self.cur() != &Token::EOF {
                    self.skip_nl();
                    if self.cur() == &Token::RBrace { break; }
                    let k = self.parse_expr();
                    self.expect(&Token::Colon);
                    let v = self.parse_expr();
                    pairs.push((k, v));
                    if self.cur() == &Token::Comma { self.adv(); }
                }
                self.expect(&Token::RBrace); Expr::MapLit(pairs)
            }
            Token::Minus => { self.adv(); let e=self.parse_primary(); Expr::UnaryOp { op: UnaryOpKind::Neg, expr: Box::new(e) } }
            _ => { self.adv(); Expr::NilLit }
        }
    }
    fn parse_expr(&mut self) -> Expr { self.parse_range() }
    fn parse_range(&mut self) -> Expr {
        let left = self.parse_or();
        if self.cur() == &Token::DotDot {
            self.adv();
            let inclusive = if self.cur() == &Token::Dot { self.adv(); true } else { false };
            let right = self.parse_or();
            Expr::Range { start: Box::new(left), end: Box::new(right), inclusive }
        } else if self.cur() == &Token::DotDotDot {
            self.adv();
            let right = self.parse_or();
            Expr::Range { start: Box::new(left), end: Box::new(right), inclusive: true }
        } else { left }
    }
    fn parse_or(&mut self) -> Expr {
        let mut l = self.parse_and();
        while self.cur() == &Token::PipePipe {
            self.adv(); let r=self.parse_and();
            l = Expr::BinOp { left: Box::new(l), op: BinOpKind::Or, right: Box::new(r) };
        }
        l
    }
    fn parse_and(&mut self) -> Expr {
        let mut l = self.parse_cmp();
        while self.cur() == &Token::AmpAmp {
            self.adv(); let r=self.parse_cmp();
            l = Expr::BinOp { left: Box::new(l), op: BinOpKind::And, right: Box::new(r) };
        }
        l
    }
    fn parse_cmp(&mut self) -> Expr {
        let mut l = self.parse_add();
        loop {
            let op = match self.cur() {
                Token::EqEq=>BinOpKind::Eq, Token::BangEq=>BinOpKind::NotEq,
                Token::Greater=>BinOpKind::Gt, Token::Less=>BinOpKind::Lt,
                Token::GreaterEq=>BinOpKind::GtEq, Token::LessEq=>BinOpKind::LtEq,
                _ => break,
            };
            self.adv(); let r=self.parse_add();
            l = Expr::BinOp { left: Box::new(l), op, right: Box::new(r) };
        }
        l
    }
    fn parse_add(&mut self) -> Expr {
        let mut l = self.parse_mul();
        loop {
            let op = match self.cur() {
                Token::Plus=>BinOpKind::Add, Token::Minus=>BinOpKind::Sub, _ => break,
            };
            self.adv(); let r=self.parse_mul();
            l = Expr::BinOp { left: Box::new(l), op, right: Box::new(r) };
        }
        l
    }
    fn parse_mul(&mut self) -> Expr {
        let mut l = self.parse_primary();
        loop {
            let op = match self.cur() {
                Token::Star=>BinOpKind::Mul, Token::Slash=>BinOpKind::Div,
                Token::Percent=>BinOpKind::Mod, Token::StarStar=>BinOpKind::Pow, _ => break,
            };
            self.adv(); let r=self.parse_primary();
            l = Expr::BinOp { left: Box::new(l), op, right: Box::new(r) };
        }
        l
    }
    fn parse_stmt(&mut self) -> Option<Stmt> {
        self.skip_nl();
        match self.cur().clone() {
            Token::Let | Token::Fix | Token::Bind => {
                self.adv();
                let name = if let Token::Identifier(n) = self.cur().clone() { self.adv(); n } else { return None; };
                let type_ann = if self.cur() == &Token::Colon { self.adv(); Some(self.parse_type()) } else { None };
                let value = if self.cur() == &Token::Eq { self.adv(); Some(self.parse_expr()) } else { None };
                Some(Stmt::Let { name, type_ann, value, mutable: true })
            }
            Token::If => {
                self.adv();
                let cond = self.parse_expr();
                let body = self.parse_block();
                let mut elif_branches = Vec::new();
                let mut else_body = None;
                loop {
                    self.skip_nl();
                    if self.cur() == &Token::Elif { self.adv(); let ec=self.parse_expr(); let eb=self.parse_block(); elif_branches.push((ec,eb)); }
                    else if self.cur() == &Token::Else { self.adv(); else_body=Some(self.parse_block()); break; }
                    else { break; }
                }
                Some(Stmt::If { cond, body, elif_branches, else_body })
            }
            Token::While => { self.adv(); let c=self.parse_expr(); let b=self.parse_block(); Some(Stmt::While { cond:c, body:b }) }
            Token::Loop  => { self.adv(); let b=self.parse_block(); Some(Stmt::Loop { body:b }) }
            Token::Each  => {
                self.adv();
                let var = if let Token::Identifier(n) = self.cur().clone() { self.adv(); n } else { "_".into() };
                if let Token::Identifier(k) = self.cur().clone() { if k=="in" { self.adv(); } }
                let iter = self.parse_expr();
                let body = self.parse_block();
                Some(Stmt::Each { var, iter, body })
            }
            Token::Fn => {
                self.adv();
                let name = if let Token::Identifier(n) = self.cur().clone() { self.adv(); n } else { return None; };
                self.expect(&Token::LParen);
                let mut params = Vec::new();
                while self.cur() != &Token::RParen && self.cur() != &Token::EOF {
                    let pn = if let Token::Identifier(n) = self.cur().clone() { self.adv(); n } else { break; };
                    let pt = if self.cur() == &Token::Colon { self.adv(); self.parse_type() } else { "any".into() };
                    params.push((pn, pt));
                    if self.cur() == &Token::Comma { self.adv(); }
                }
                self.expect(&Token::RParen);
                let return_type = if self.cur() == &Token::Arrow { self.adv(); Some(self.parse_type()) } else { None };
                let body = self.parse_block();
                Some(Stmt::Fn { name, params, return_type, body, is_async: false })
            }
            Token::Async => {
                self.adv();
                if self.cur() == &Token::Fn {
                    self.adv();
                    let name = if let Token::Identifier(n) = self.cur().clone() { self.adv(); n } else { return None; };
                    self.expect(&Token::LParen);
                    let mut params = Vec::new();
                    while self.cur() != &Token::RParen && self.cur() != &Token::EOF {
                        let pn = if let Token::Identifier(n) = self.cur().clone() { self.adv(); n } else { break; };
                        let pt = if self.cur() == &Token::Colon { self.adv(); self.parse_type() } else { "any".into() };
                        params.push((pn, pt));
                        if self.cur() == &Token::Comma { self.adv(); }
                    }
                    self.expect(&Token::RParen);
                    let return_type = if self.cur() == &Token::Arrow { self.adv(); Some(self.parse_type()) } else { None };
                    let body = self.parse_block();
                    Some(Stmt::Fn { name, params, return_type, body, is_async: true })
                } else { None }
            }
            Token::Class => {
                self.adv();
                let name = if let Token::Identifier(n) = self.cur().clone() { self.adv(); n } else { return None; };
                let body = self.parse_block();
                Some(Stmt::Class { name, body })
            }
            Token::Try => {
                self.adv(); let body=self.parse_block(); self.skip_nl();
                let (cv, cb) = if self.cur() == &Token::Catch {
                    self.adv();
                    let v = if let Token::Identifier(n) = self.cur().clone() { self.adv(); n } else { "err".into() };
                    (v, self.parse_block())
                } else { ("err".into(), Vec::new()) };
                Some(Stmt::TryCatch { body, catch_var: cv, catch_body: cb })
            }
            Token::Match => {
                self.adv(); let expr=self.parse_expr(); self.skip_nl(); self.expect(&Token::LBrace);
                let mut arms = Vec::new();
                loop {
                    self.skip_nl();
                    if self.cur() == &Token::RBrace || self.cur() == &Token::EOF { break; }
                    let pat = self.parse_expr(); self.skip_nl();
                    if self.cur() == &Token::FatArrow { self.adv(); }
                    let ab = self.parse_block();
                    arms.push((pat, ab));
                }
                self.expect(&Token::RBrace);
                Some(Stmt::Match { expr, arms })
            }
            Token::Give => {
                self.adv();
                if self.cur() == &Token::Newline || self.cur() == &Token::EOF { Some(Stmt::Give(None)) }
                else { Some(Stmt::Give(Some(self.parse_expr()))) }
            }
            Token::Use => {
                self.adv();
                let mut path = Vec::new();
                if let Token::Identifier(n) = self.cur().clone() { self.adv(); path.push(n); }
                while self.cur() == &Token::Dot {
                    self.adv();
                    if let Token::Identifier(n) = self.cur().clone() { self.adv(); path.push(n); }
                }
                Some(Stmt::Use { path, alias: None })
            }
            Token::Break => { self.adv(); Some(Stmt::Break) }
            Token::Skip  => { self.adv(); Some(Stmt::Skip)  }
            Token::EOF | Token::Newline => { self.adv(); None }
            _ => {
                let expr = self.parse_expr();
                // Check if assignment: ident = value
                if let Expr::Ident(ref name) = expr {
                    if self.cur() == &Token::Eq {
                        let n = name.clone();
                        self.adv();
                        let value = self.parse_expr();
                        return Some(Stmt::Assign { name: n, value });
                    }
                }
                Some(Stmt::ExprStmt(expr))
            }
        }
    }
    fn parse_block(&mut self) -> Vec<Stmt> {
        self.skip_nl(); self.expect(&Token::LBrace);
        let mut stmts = Vec::new();
        loop {
            self.skip_nl();
            if self.cur() == &Token::RBrace || self.cur() == &Token::EOF { break; }
            if let Some(s) = self.parse_stmt() { stmts.push(s); }
        }
        self.expect(&Token::RBrace); stmts
    }
    pub fn parse(&mut self) -> Program {
        let mut stmts = Vec::new();
        loop {
            self.skip_nl();
            if self.cur() == &Token::EOF { break; }
            if let Some(s) = self.parse_stmt() { stmts.push(s); }
        }
        Program { stmts }
    }
}
