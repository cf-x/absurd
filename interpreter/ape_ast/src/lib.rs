use ape_expr::Expression;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    /// !
    Not,
    /// !!
    NotNot,
    /// ~
    Tilde,
    /// %
    Percent,
    /// &
    And,
    /// &&
    AndAnd,
    /// *
    Mult,
    /// **
    Square,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// -
    Minus,
    /// --
    Decr,
    /// ->
    Arrow,
    /// =>
    ArrowBig,
    /// _
    Underscore,
    /// +
    Plus,
    /// ++
    Increment,
    /// =
    Assign,
    /// ==
    Eq,
    /// !=
    NotEq,
    /// +=
    PlusEq,
    /// -=
    MinEq,
    /// *=
    MultEq,
    /// /=
    DivEq,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// ;
    Semi,
    /// :
    Colon,
    /// ::
    DblColon,
    /// char
    CharLit,
    /// string
    StringLit,
    /// number
    NumberLit,
    /// true
    TrueLit,
    /// false
    FalseLit,
    /// null
    NullLit,
    /// <
    Less,
    /// <=
    LessOrEq,
    /// >
    Greater,
    /// >=
    GreaterOrEq,
    /// ,
    Comma,
    /// .
    Dot,
    /// ..
    DotDot,
    /// ...
    Spread,
    /// /
    Divide,
    /// \
    Escape,
    /// \{
    StartParse,
    /// \}
    EndParse,
    /// ?
    Queston,
    /// |
    Pipe,
    /// ||
    Or,
    /// identifier
    Ident,
    /// end of file
    Eof,
    /// variable (let)
    Let,
    /// if
    If,
    /// else
    Else,
    /// else if
    ElseIf,
    /// return
    Return,
    /// while
    While,
    /// loop
    Loop,
    /// break
    Break,
    /// match
    Match,
    /// mod
    Mod,
    /// use
    Use,
    /// as
    As,
    /// from
    From,
    /// struct
    Struct,
    /// self
    Slf,
    /// impl
    Impl,
    /// enum
    Enum,
    /// async
    Async,
    /// await
    Await,
    /// pub
    Pub,
    /// mut
    Mut,
    /// function
    Func,
    /// number
    NumberIdent,
    /// string
    StringIdent,
    /// char
    CharIdent,
    /// bool
    BoolIdent,
    /// null
    NullIdent,
    /// void
    VoidIdent,
    /// array(type)
    ArrayIdent,
    /// any
    AnyIdent,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FuncValueType {
    Func,
    Std,
    Callback,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Base {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralType {
    Number(f32),
    String(String),
    Char(char),
    Boolean(bool),
    Null,
    Void,
    Any,
    Array(Vec<LiteralType>),
    Func(FuncValueType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Number { base: Base, value: f32 },
    String { value: String },
    Char { value: char },
    Bool { value: bool },
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub value: Option<LiteralKind>,
    pub line: usize,
    pub pos: (usize, usize),
}


#[derive(Debug, PartialEq, Clone)]
pub enum CallType {
    Func,
    Var,
    Struct,
    OpenStruct,
    Method,
    Enum,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression {
        expr: Expression,
    },
    Block {
        stmts: Vec<Statement>,
    },
    Var {
        names: Vec<Token>,
        value_type: Token,
        value: Option<Expression>,
        is_mut: bool,
        is_pub: bool,
        pub_names: Vec<Token>,
        is_func: bool,
    },
    Func {
        name: Token,
        value_type: Token,
        body: FuncBody,
        params: Vec<(Token, Token)>,
        is_async: bool,
        is_pub: bool,
        is_impl: bool,
        is_mut: bool,
    },
    If {
        cond: Expression,
        body: Vec<Statement>,
        else_if_branches: Vec<(Expression, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    },
    Return {
        expr: Expression,
    },
    While {
        cond: Expression,
        body: Vec<Statement>,
    },
    Loop {
        iter: Option<usize>,
        body: Vec<Statement>,
    },
    Break {},
    Match {
        cond: Expression,
        cases: Vec<(Expression, FuncBody)>,
        def_case: FuncBody,
    },
    Mod {
        src: String,
    },
    Use {
        src: String,
        names: Vec<(Token, Option<Token>)>,
    },
    Struct {
        name: Token,
        structs: Vec<(Token, TokenType, bool)>,
        is_pub: bool,
        methods: Vec<(Expression, bool)>,
    },
    Impl {
        name: Token,
        body: Vec<Statement>,
    },
    Enum {
        name: Token,
        enums: Vec<Token>,
        is_pub: bool,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum FuncBody {
    Statements(Vec<Statement>),
    Expression(Box<Expression>),
}
