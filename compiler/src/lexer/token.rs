#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Control Flow
    If, Else, Elif, Match, When,
    Loop, While, Each, Break, Skip,
    Give, Halt,
    // Functions & Classes
    Fn, Method, Class, Soul, Base,
    Trait, Impl, Build, Drop, Init,
    // Memory & Safety
    Own, Borrow, Ref, Flex, Freeze,
    Safe, Raw, Scope,
    // Data Declaration
    Let, Fix, Bind, Kind, Alias,
    Block, Group, Choice, Pack, Label,
    // Async
    Async, Await, Spawn, PipeKw,
    Send, Recv, Lock, Atomic,
    // Modules
    Mod, Use, Expose, Hide, From, As,
    // Error Handling
    Try, Catch, Toss, Rescue,
    Guard, Ensure, Panic,
    // AI
    Model, Infer, Teach, Tensor,
    Flow, Tokenize, Embed, Predict,
    // Backend
    Route, Serve, Req, Res,
    Layer, Socket, Stream, Endpoint,
    // Misc
    True, False, Null, Void,
    This, Super, Kindof, Sizof,
    // Data Types
    Tiny, Short, Ant, Long, Vast,
    Utiny, Ushort, Uant, Ulong,
    Half, Dbl, Precise,
    Chr, Str, Txt,
    Bool, Nil,
    List, Map, Set,
    TensorType, Blob,
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),
    // Identifier
    Identifier(String),
    // Arithmetic
    Plus, Minus, Star, Slash, Percent, StarStar,
    // Comparison
    EqEq, BangEq, Greater, Less, GreaterEq, LessEq,
    // Logical
    AmpAmp, PipePipe, BangBang,
    // Bitwise
    Amp, Pipe, Caret, Tilde, LtLt, GtGt,
    // Assignment
    Eq, PlusEq, MinusEq, StarEq, SlashEq,
    PercentEq, StarStarEq, ColonEq,
    // Pipeline
    PipeArrow, ArrowPipe, FatArrow, Arrow,
    // Safety
    QuestionQuestion, QuestionColon, BangQuestion,
    // Memory
    At, ColonColon, TildeArrow, HashHash,
    // Range
    DotDot, DotDotDot,
    // Delimiters
    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,
    Semicolon, Colon, Comma, Dot,
    // Special
    Newline,
    EOF,
    Unknown(char),
}
