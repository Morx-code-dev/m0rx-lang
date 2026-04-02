pub mod ast;
use ast::*;
use crate::lexer::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> &Token {
        let tok = self.tokens.get(self.pos).unwrap_or(&Token::EOF);
        self.pos += 1;
        tok
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos + 1).unwrap_or(&Token::EOF)
    }

    fn skip_newlines(&mut self) {
        while self.current() == &Token::Newline {
            self.advance();
        }
    }

    fn expect(&mut self, tok: &Token) -> bool {
        if self.current() == tok {
            self.advance();
            true
        } else {
            eprintln!(
                "M0RX Parse Error: expected {:?}, got {:?}",
                tok,
                self.current()
            );
            false
        }
    }

    // Parse primary expression
    fn parse_primary(&mut self) -> Expr {
        match self.current().clone() {
            Token::IntLiteral(n) => {
                let v = n;
                self.advance();
                Expr::IntLit(v)
            }
            Token::FloatLiteral(f) => {
                let v = f;
                self.advance();
                Expr::FloatLit(v)
            }
            Token::StringLiteral(s) => {
                let v = s.clone();
                self.advance();
                Expr::StrLit(v)
            }
            Token::True => {
                self.advance();
                Expr::BoolLit(true)
            }
            Token::False => {
                self.advance();
                Expr::BoolLit(false)
            }
            Token::Null => {
                self.advance();
                Expr::NilLit
            }
            Token::Identifier(name) => {
                let n = name.clone();
                self.advance();
                // Function call?
                if self.current() == &Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current() != &Token::RParen
                        && self.current() != &Token::EOF
                    {
                        args.push(self.parse_expr());
                        if self.current() == &Token::Comma {
                            self.advance();
                        }
                    }
                    self.expect(&Token::RParen);
                    Expr::Call { name: n, args }
                } else {
                    Expr::Ident(n)
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(&Token::RParen);
                expr
            }
            Token::LBracket => {
                self.advance();
                let mut items = Vec::new();
                while self.current() != &Token::RBracket
                    && self.current() != &Token::EOF
                {
                    items.push(self.parse_expr());
                    if self.current() == &Token::Comma {
                        self.advance();
                    }
                }
                self.expect(&Token::RBracket);
                Expr::ListLit(items)
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_primary();
                Expr::UnaryOp {
                    op: UnaryOpKind::Neg,
                    expr: Box::new(expr),
                }
            }
            _ => {
                self.advance();
                Expr::NilLit
            }
        }
    }

    // Parse binary expression with precedence
    fn parse_expr(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();
        while self.current() == &Token::PipePipe {
            self.advance();
            let right = self.parse_and();
            left = Expr::BinOp {
                left: Box::new(left),
                op: BinOpKind::Or,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_comparison();
        while self.current() == &Token::AmpAmp {
            self.advance();
            let right = self.parse_comparison();
            left = Expr::BinOp {
                left: Box::new(left),
                op: BinOpKind::And,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_addition();
        loop {
            let op = match self.current() {
                Token::EqEq => BinOpKind::Eq,
                Token::BangEq => BinOpKind::NotEq,
                Token::Greater => BinOpKind::Gt,
                Token::Less => BinOpKind::Lt,
                Token::GreaterEq => BinOpKind::GtEq,
                Token::LessEq => BinOpKind::LtEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_addition();
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_addition(&mut self) -> Expr {
        let mut left = self.parse_multiplication();
        loop {
            let op = match self.current() {
                Token::Plus => BinOpKind::Add,
                Token::Minus => BinOpKind::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplication();
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_multiplication(&mut self) -> Expr {
        let mut left = self.parse_primary();
        loop {
            let op = match self.current() {
                Token::Star => BinOpKind::Mul,
                Token::Slash => BinOpKind::Div,
                Token::Percent => BinOpKind::Mod,
                Token::StarStar => BinOpKind::Pow,
                _ => break,
            };
            self.advance();
            let right = self.parse_primary();
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        left
    }

    // Parse statement
    fn parse_stmt(&mut self) -> Option<Stmt> {
        self.skip_newlines();
        match self.current().clone() {
            Token::Let | Token::Fix | Token::Bind => {
                self.advance();
                let mutable = true;
                let name = if let Token::Identifier(n) = self.current().clone() {
                    self.advance();
                    n
                } else {
                    return None;
                };
                let type_ann = if self.current() == &Token::Colon {
                    self.advance();
                    if let Token::Identifier(t) = self.current().clone() {
                        self.advance();
                        Some(t)
                    } else {
                        None
                    }
                } else {
                    None
                };
                let value = if self.current() == &Token::Eq {
                    self.advance();
                    Some(self.parse_expr())
                } else {
                    None
                };
                Some(Stmt::Let { name, type_ann, value, mutable })
            }

            Token::If => {
                self.advance();
                let cond = self.parse_expr();
                let body = self.parse_block();
                let mut elif_branches = Vec::new();
                let mut else_body = None;
                loop {
                    self.skip_newlines();
                    if self.current() == &Token::Elif {
                        self.advance();
                        let ec = self.parse_expr();
                        let eb = self.parse_block();
                        elif_branches.push((ec, eb));
                    } else if self.current() == &Token::Else {
                        self.advance();
                        else_body = Some(self.parse_block());
                        break;
                    } else {
                        break;
                    }
                }
                Some(Stmt::If { cond, body, elif_branches, else_body })
            }

            Token::While => {
                self.advance();
                let cond = self.parse_expr();
                let body = self.parse_block();
                Some(Stmt::While { cond, body })
            }

            Token::Loop => {
                self.advance();
                let body = self.parse_block();
                Some(Stmt::Loop { body })
            }

            Token::Each => {
                self.advance();
                let var = if let Token::Identifier(n) = self.current().clone() {
                    self.advance();
                    n
                } else {
                    String::from("_")
                };
                // skip 'in'
                if let Token::Identifier(kw) = self.current().clone() {
                    if kw == "in" { self.advance(); }
                }
                let iter = self.parse_expr();
                let body = self.parse_block();
                Some(Stmt::Each { var, iter, body })
            }

            Token::Fn => {
                self.advance();
                let name = if let Token::Identifier(n) = self.current().clone() {
                    self.advance();
                    n
                } else {
                    return None;
                };
                self.expect(&Token::LParen);
                let mut params = Vec::new();
                while self.current() != &Token::RParen
                    && self.current() != &Token::EOF
                {
                    let pname = if let Token::Identifier(n) = self.current().clone() {
                        self.advance();
                        n
                    } else { break; };
                    let ptype = if self.current() == &Token::Colon {
                        self.advance();
                        if let Token::Identifier(t) = self.current().clone() {
                            self.advance();
                            t
                        } else { String::from("any") }
                    } else { String::from("any") };
                    params.push((pname, ptype));
                    if self.current() == &Token::Comma {
                        self.advance();
                    }
                }
                self.expect(&Token::RParen);
                let return_type = if self.current() == &Token::Arrow {
                    self.advance();
                    if let Token::Identifier(t) = self.current().clone() {
                        self.advance();
                        Some(t)
                    } else { None }
                } else { None };
                let body = self.parse_block();
                Some(Stmt::Fn { name, params, return_type, body, is_async: false })
            }

            Token::Give => {
                self.advance();
                if self.current() == &Token::Newline
                    || self.current() == &Token::EOF
                {
                    Some(Stmt::Give(None))
                } else {
                    Some(Stmt::Give(Some(self.parse_expr())))
                }
            }

            Token::Break => {
                self.advance();
                Some(Stmt::Break)
            }

            Token::Skip => {
                self.advance();
                Some(Stmt::Skip)
            }

            Token::Use => {
                self.advance();
                let mut path = Vec::new();
                if let Token::Identifier(n) = self.current().clone() {
                    self.advance();
                    path.push(n);
                }
                while self.current() == &Token::Dot {
                    self.advance();
                    if let Token::Identifier(n) = self.current().clone() {
                        self.advance();
                        path.push(n);
                    }
                }
                Some(Stmt::Use { path, alias: None })
            }

            Token::EOF => None,
            Token::Newline => {
                self.advance();
                None
            }

            _ => {
                let expr = self.parse_expr();
                Some(Stmt::ExprStmt(expr))
            }
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.skip_newlines();
        self.expect(&Token::LBrace);
        let mut stmts = Vec::new();
        loop {
            self.skip_newlines();
            if self.current() == &Token::RBrace
                || self.current() == &Token::EOF
            {
                break;
            }
            if let Some(s) = self.parse_stmt() {
                stmts.push(s);
            }
        }
        self.expect(&Token::RBrace);
        stmts
    }

    pub fn parse(&mut self) -> Program {
        let mut stmts = Vec::new();
        loop {
            self.skip_newlines();
            if self.current() == &Token::EOF {
                break;
            }
            if let Some(s) = self.parse_stmt() {
                stmts.push(s);
            }
        }
        Program { stmts }
    }
}
