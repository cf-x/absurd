pub mod literals;
use std::{
    cell::RefCell,
    fmt::{self, Debug},
    rc::Rc,
};
pub mod token;
use crate::interpreter::{env::Env, expr::Expression, types::TypeKind};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    /// !
    Bang,
    /// !!
    DblBang,
    /// %
    Prcnt,
    /// &
    And,
    /// &&
    DblAnd,
    /// `*`
    Mul,
    /// **
    Sqr,
    /// (
    LParen,
    /// )
    RParen,
    /// -
    Min,
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
    Incr,
    /// =
    Assign,
    /// ==
    Eq,
    /// !=
    BangEq,
    /// +=
    PlusEq,
    /// -=
    MinEq,
    /// *=
    MulEq,
    /// /=
    DivEq,
    /// {
    LBrace,
    /// }
    RBrace,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// ;
    Semi,
    /// :
    Colon,
    /// ::
    DblColon,
    /// char
    CharLit,
    /// string
    StrLit,
    /// number
    NumLit,
    /// true
    TrueLit,
    /// false
    FalseLit,
    /// <
    Ls,
    /// <=
    LsOrEq,
    /// >
    Gr,
    /// >=
    GrOrEq,
    /// ,
    Comma,
    /// .
    Dot,
    /// ..
    DblDot,
    /// /
    Div,
    /// \
    Esc,
    /// \{
    LParse,
    /// \}
    RParse,
    /// ?
    Qstn,
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
    /// elif
    Elif,
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
    /// type
    TypeStmt,
    /// sh
    Sh,
    /// number
    NumIdent,
    /// string
    StrIdent,
    /// char
    CharIdent,
    /// bool
    BoolIdent,
    /// null
    Null,
    /// void
    VoidIdent,
    /// array
    ArrayIdent,
    // any type
    Type,
    // callback type
    FuncIdent,
    /// any
    AnyIdent,
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
    Vec(Vec<LiteralType>),
    Obj(Vec<(String, Expression)>),
    Func(FuncImpl),
    DeclrFunc(DeclrFuncType),
}

#[derive(Debug, Clone)]
pub struct DeclrFuncType {
    pub name: String,
    pub arity: usize,
    pub func: Rc<dyn FuncValType>,
}

pub trait FuncValType {
    fn call(&self, args: Vec<Option<LiteralType>>) -> LiteralType;
}

impl Debug for dyn FuncValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FuncValType")
    }
}

impl<'a> PartialEq for DeclrFuncType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity && self.func.rc_eq(&other.func)
    }
}

pub trait RcFuncValType {
    fn rc_eq(&self, other: &Self) -> bool;
}

impl RcFuncValType for Rc<dyn FuncValType> {
    fn rc_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

pub struct Wrapper(pub Box<dyn Fn(&[Option<LiteralType>]) -> LiteralType>);

impl FuncValType for Wrapper {
    fn call(&self, args: Vec<Option<LiteralType>>) -> LiteralType {
        (self.0)(&args)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Number { base: Base, value: f32 },
    String { value: String },
    Char { value: char },
    Bool { value: bool },
    Type(Box<TypeKind>),
    Null,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FuncImpl {
    pub name: String,
    pub value_type: Token,
    pub body: FuncBody,
    pub params: Vec<(Token, Token)>,
    pub is_async: bool,
    pub is_pub: bool,
    pub env: Rc<RefCell<Env>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub value: Option<LiteralKind>,
    pub line: usize,
    pub pos: (usize, usize),
}

impl Token {
    pub fn token_to_typekind(&mut self) -> TypeKind {
        match self.clone().value {
            Some(LiteralKind::Type(t)) => *t,
            _ => TypeKind::Var { name: self.clone() },
        }
    }
    pub fn null() -> Self {
        Token {
            token: TokenType::Null,
            lexeme: "null".to_string(),
            value: None,
            line: 0,
            pos: (0, 0),
        }
    }
    pub fn empty(token: TokenType, lexeme: &str, value: Option<LiteralKind>) -> Self {
        Token {
            token,
            lexeme: lexeme.to_string(),
            value,
            line: 0,
            pos: (0, 0),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CallType {
    Func,
    Struct,
    Enum,
    Array,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Sh {
        cmd: String,
    },
    Type {
        name: Token,
        is_pub: bool,
        value: Token,
    },
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
        is_arr_dest: bool,
    },
    Func {
        name: Token,
        value_type: Token,
        body: FuncBody,
        params: Vec<(Token, Token)>,
        is_async: bool,
        is_pub: bool,
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
        all: bool,
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
