pub mod token;
use token::Token;

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current();
        self.pos += 1;
        self.col += 1;
        if ch == Some('\n') {
            self.line += 1;
            self.col = 1;
        }
        ch
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.pos + 1).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        if self.current() == Some('/') && self.peek() == Some('/') {
            while let Some(ch) = self.current() {
                if ch == '\n' { break; }
                self.advance();
            }
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance();
        let mut s = String::new();
        while let Some(ch) = self.current() {
            if ch == '"' { self.advance(); break; }
            s.push(ch);
            self.advance();
        }
        Token::StringLiteral(s)
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        let mut is_float = false;
        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() {
                num.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                is_float = true;
                num.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if is_float {
            Token::FloatLiteral(num.parse().unwrap_or(0.0))
        } else {
            Token::IntLiteral(num.parse().unwrap_or(0))
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        match_keyword(&ident)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            self.skip_comment();
            match self.current() {
                None => { tokens.push(Token::EOF); break; }
                Some('\n') => {
                    tokens.push(Token::Newline);
                    self.advance();
                }
                Some('"') => tokens.push(self.read_string()),
                Some(ch) if ch.is_ascii_digit() => {
                    tokens.push(self.read_number());
                }
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    tokens.push(self.read_identifier());
                }
                Some(_) => {
                    let tok = self.read_operator();
                    tokens.push(tok);
                }
            }
        }
        tokens
    }

    fn read_operator(&mut self) -> Token {
        let ch = self.advance().unwrap_or('\0');
        let next = self.current();
        match (ch, next) {
            ('+', Some('=')) => { self.advance(); Token::PlusEq }
            ('+', _) => Token::Plus,
            ('-', Some('=')) => { self.advance(); Token::MinusEq }
            ('-', Some('>')) => { self.advance(); Token::Arrow }
            ('-', _) => Token::Minus,
            ('*', Some('*')) => { self.advance(); Token::StarStar }
            ('*', Some('=')) => { self.advance(); Token::StarEq }
            ('*', _) => Token::Star,
            ('/', Some('=')) => { self.advance(); Token::SlashEq }
            ('/', _) => Token::Slash,
            ('%', Some('=')) => { self.advance(); Token::PercentEq }
            ('%', _) => Token::Percent,
            ('=', Some('=')) => { self.advance(); Token::EqEq }
            ('=', Some('>')) => { self.advance(); Token::FatArrow }
            ('=', _) => Token::Eq,
            ('!', Some('=')) => { self.advance(); Token::BangEq }
            ('!', Some('?')) => { self.advance(); Token::BangQuestion }
            ('!', Some('!')) => { self.advance(); Token::BangBang }
            ('!', _) => Token::BangBang,
            ('>', Some('=')) => { self.advance(); Token::GreaterEq }
            ('>', Some('>')) => { self.advance(); Token::GtGt }
            ('>', _) => Token::Greater,
            ('<', Some('=')) => { self.advance(); Token::LessEq }
            ('<', Some('<')) => { self.advance(); Token::LtLt }
            ('<', Some('|')) => { self.advance(); Token::ArrowPipe }
            ('<', _) => Token::Less,
            ('&', Some('&')) => { self.advance(); Token::AmpAmp }
            ('&', _) => Token::Amp,
            ('|', Some('|')) => { self.advance(); Token::PipePipe }
            ('|', Some('>')) => { self.advance(); Token::PipeArrow }
            ('|', _) => Token::Pipe,
            ('^', _) => Token::Caret,
            ('~', Some('>')) => { self.advance(); Token::TildeArrow }
            ('~', _) => Token::Tilde,
            ('@', _) => Token::At,
            ('#', Some('#')) => { self.advance(); Token::HashHash }
            (':', Some(':')) => { self.advance(); Token::ColonColon }
            (':', Some('=')) => { self.advance(); Token::ColonEq }
            (':', _) => Token::Colon,
            ('?', Some('?')) => { self.advance(); Token::QuestionQuestion }
            ('?', Some(':')) => { self.advance(); Token::QuestionColon }
            ('.', Some('.')) => {
                self.advance();
                if self.current() == Some('.') {
                    self.advance();
                    Token::DotDotDot
                } else {
                    Token::DotDot
                }
            }
            ('.', _) => Token::Dot,
            ('(', _) => Token::LParen,
            (')', _) => Token::RParen,
            ('{', _) => Token::LBrace,
            ('}', _) => Token::RBrace,
            ('[', _) => Token::LBracket,
            (']', _) => Token::RBracket,
            (';', _) => Token::Semicolon,
            (',', _) => Token::Comma,
            (c, _) => Token::Unknown(c),
        }
    }
}

fn match_keyword(ident: &str) -> Token {
    match ident {
        "if" => Token::If,
        "else" => Token::Else,
        "elif" => Token::Elif,
        "match" => Token::Match,
        "when" => Token::When,
        "loop" => Token::Loop,
        "while" => Token::While,
        "each" => Token::Each,
        "break" => Token::Break,
        "skip" => Token::Skip,
        "give" => Token::Give,
        "halt" => Token::Halt,
        "fn" => Token::Fn,
        "method" => Token::Method,
        "class" => Token::Class,
        "soul" => Token::Soul,
        "base" => Token::Base,
        "trait" => Token::Trait,
        "impl" => Token::Impl,
        "build" => Token::Build,
        "drop" => Token::Drop,
        "init" => Token::Init,
        "own" => Token::Own,
        "borrow" => Token::Borrow,
        "ref" => Token::Ref,
        "flex" => Token::Flex,
        "freeze" => Token::Freeze,
        "safe" => Token::Safe,
        "raw" => Token::Raw,
        "scope" => Token::Scope,
        "let" => Token::Let,
        "fix" => Token::Fix,
        "bind" => Token::Bind,
        "kind" => Token::Kind,
        "alias" => Token::Alias,
        "block" => Token::Block,
        "group" => Token::Group,
        "choice" => Token::Choice,
        "pack" => Token::Pack,
        "label" => Token::Label,
        "async" => Token::Async,
        "await" => Token::Await,
        "spawn" => Token::Spawn,
        "pipe" => Token::PipeKw,
        "send" => Token::Send,
        "recv" => Token::Recv,
        "lock" => Token::Lock,
        "atomic" => Token::Atomic,
        "mod" => Token::Mod,
        "use" => Token::Use,
        "expose" => Token::Expose,
        "hide" => Token::Hide,
        "from" => Token::From,
        "as" => Token::As,
        "try" => Token::Try,
        "catch" => Token::Catch,
        "toss" => Token::Toss,
        "rescue" => Token::Rescue,
        "guard" => Token::Guard,
        "ensure" => Token::Ensure,
        "panic" => Token::Panic,
        "model" => Token::Model,
        "infer" => Token::Infer,
        "teach" => Token::Teach,
        "tensor" => Token::Tensor,
        "flow" => Token::Flow,
        "tokenize" => Token::Tokenize,
        "embed" => Token::Embed,
        "predict" => Token::Predict,
        "route" => Token::Route,
        "serve" => Token::Serve,
        "req" => Token::Req,
        "res" => Token::Res,
        "layer" => Token::Layer,
        "socket" => Token::Socket,
        "stream" => Token::Stream,
        "endpoint" => Token::Endpoint,
        "true" => Token::True,
        "false" => Token::False,
        "null" => Token::Null,
        "void" => Token::Void,
        "this" => Token::This,
        "super" => Token::Super,
        "kindof" => Token::Kindof,
        "sizof" => Token::Sizof,
        "tiny" => Token::Tiny,
        "short" => Token::Short,
        "ant" => Token::Ant,
        "long" => Token::Long,
        "vast" => Token::Vast,
        "utiny" => Token::Utiny,
        "ushort" => Token::Ushort,
        "uant" => Token::Uant,
        "ulong" => Token::Ulong,
        "half" => Token::Half,
        "dbl" => Token::Dbl,
        "precise" => Token::Precise,
        "chr" => Token::Chr,
        "str" => Token::Str,
        "txt" => Token::Txt,
        "bool" => Token::Bool,
        "nil" => Token::Nil,
        "list" => Token::List,
        "map" => Token::Map,
        "set" => Token::Set,
        "blob" => Token::Blob,
        _ => Token::Identifier(ident.to_string()),
    }
}
