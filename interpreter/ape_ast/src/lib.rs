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
    // 0b prefix
    Binary = 2,
    // 0o prefix
    Octal = 8,
    // no prefix
    Decimal = 10,
    // 0x prefix
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
    pub len: u32,
    pub lexeme: String,
    pub value: Option<LiteralKind>,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Array {
        id: usize,
        items: Vec<LiteralType>,
    },
    Var {
        id: usize,
        name: Token,
    },
    Call {
        id: usize,
        // identifier
        name: Box<Token>,
        args: Vec<Expression>,
        call_type: CallType,
    },
    // expression operator
    Unary {
        id: usize,
        left: Box<Expression>,
        operator: Token,
    },
    // expression operator expression
    Binary {
        id: usize,
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    // (expression)
    Grouping {
        id: usize,
        expression: Box<Expression>,
    },
    Value {
        id: usize,
        value: LiteralType,
    },
    Func {
        id: usize,
        // inherited
        name: Token,
        // inherited
        value_type: Token,
        body: FuncBody,
        params: Vec<(Token, Token)>,
        is_async: bool,
        // inherited
        is_pub: bool,
        // can't be muttated or implemented
    },
    If {
        id: usize,
        cond: Box<Expression>,
        body: Vec<Statement>,
        else_if_branches: Vec<(Expression, Vec<Statement>)>,
        else_branch: Box<Statement>,
    },
    While {
        id: usize,
        cond: Box<Expression>,
        body: Vec<Statement>,
    },
    Loop {
        id: usize,
        iter: Option<usize>,
        body: Vec<Statement>,
    },
    Match {
        id: usize,
        cond: Box<Expression>,
        cases: Vec<(Expression, FuncBody)>,
        def_case: FuncBody,
    },
    Await {
        id: usize,
        expr: Box<Expression>,
    },
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
        // identifiers
        names: Vec<Token>,
        // type tokens
        value_type: Token,
        // null, if no expression
        value: Option<Expression>,
        // can't be mutable and public simultaniously
        is_mut: bool,
        is_pub: bool,
        // should match number of names, if non-empty
        pub_names: Vec<Token>,
        // if has callback, is function
        is_func: bool,
    },
    Func {
        // identifier
        name: Token,
        // type token
        value_type: Token,
        // either block or expression
        body: FuncBody,
        // (identifier, type token)
        params: Vec<(Token, Token)>,
        // function can be both async and public at the same time
        is_async: bool,
        is_pub: bool,
        // if inside implementation, `self` should be included before params (not included in params)
        is_impl: bool,
        // if implementing, if `self` is mutable: `mut self`
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
        // (identifier, type token, is_public)
        structs: Vec<(Token, TokenType, bool)>,
        is_pub: bool,
        // (function name, is_public)
        methods: Vec<(Expression, bool)>,
    },
    Impl {
        name: Token,
        // functions-only
        body: Vec<Statement>,
    },
    Enum {
        // identifier
        name: Token,
        // identifiers (capitilized)
        enums: Vec<Token>,
        is_pub: bool,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum FuncBody {
    Statements(Vec<Statement>),
    Expression(Box<Expression>),
}
