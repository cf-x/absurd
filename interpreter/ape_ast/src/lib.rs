#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
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
    /// '
    CharLit,
    /// "
    StringLit,
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
    /// unknown character
    Unknown,
    /// unknown prefix
    UnknownPrefix,
    /// invalid prefix
    InvalidPrefix,
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

#[derive(Debug, PartialEq, Eq)]
pub enum FuncValueType {
    Func,
    Std,
    Callback,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq)]
pub enum LiteralType {
    // 3.1415, 663, 0b1101, 17e6
    Number(f32),
    // "john doe", "welcome back \{username\}"
    String(String),
    // 'c', '💙'
    Char(char),
    // true, false
    Boolean(bool),
    // null
    Null,
    // void
    Void,
    // any
    Any,
    // [1, 2, 3, 4, 5]
    Array(Vec<LiteralType>),
    // |a: number, b: number| a + b
    Func(FuncValueType),
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    /*
        f32 number
        supports bases:
        - 3, 4.23, 13_324 // base 10
        - 0b01 // base 2
        - 0o012345678 // base 8
        - 0x0123456789abcdef // base 16
        suports exponents:
        - 3e3 same as 3000
    */
    Number {
        base: Base,
        empty_expo: bool,
    },
    /*
       "normal string"
       supports parsing:
       - "\{expression\}"
       - "username \{name\}" same as "username john doe"
    */
    String {
        terminated: bool,
        exprs: Vec<Expression>,
    },
    /*
        'c', '💙'
    */
    Char {
        terminated: bool,
    },
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub len: u32,
    pub lexeme: String,
    pub value: Option<LiteralKind>,
    pub line: usize,
}

impl Token {
    fn new(
        token: TokenType,
        len: u32,
        lexeme: String,
        value: Option<LiteralKind>,
        line: usize,
    ) -> Token {
        Token {
            token,
            len,
            lexeme,
            value,
            line,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    // [expr, expr, epxr]
    Array {
        id: usize,
        items: Vec<LiteralType>,
    },
    Var {
        id: usize,
        name: Token,
    },
    /*
        call_function();
        call_function(arg1, arg2);
        call_function(param2: arg2, param1: arg1);
        // name: call_variable, arg: none
        call_variable;
        // name: call, arg: structure
        call.structure;
        // name: call, arg: enum
        call::enum;
        // name: call, args: enum1, enum2
        call::{enum1, enum2}
    */
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
    /*
    let name: |bool| -> bool = | init: bool | init!;
     | | {/* body */}
     | | expression
     | param: type | {}
     | async, param: type | {}
    */
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
        // can't be muttated and implemented
    },
    /*
        body must return a value

        if expr {}
        if expr {} else {}
        if expr {} else if expr {} else {}
    */
    If {
        id: usize,
        cond: Box<Expression>,
        body: FuncBody,
        else_if_branches: Vec<(Vec<Expression>, Box<Statement>)>,
        else_branch: Option<Box<Statement>>,
    },
    /*
      body must return a value

      while expr {}
    */
    While {
        id: usize,
        cond: Box<Expression>,
        body: FuncBody,
    },
    /*
      body must return a value

      loop {}
      loop iter {}
    */
    Loop {
        id: usize,
        iter: Option<usize>,
        body: FuncBody,
    },
    /*
      body must return a value

        match expr {
        value1 => {} // opt
        value2 => {} // opt
        _ => {} // req
        }
    */
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

#[derive(Debug, PartialEq)]
pub enum CallType {
    Func,
    Var,
    Struct,
    Enum,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression {
        expr: Expression,
    },
    Block {
        stmts: Vec<Statement>,
    },
    /*
    variables:

    let name: type = expression;
    let name1, name2: type = expression;
    let name;
    let mut name: type = expression;
    let pub name: type = expression;
    let pub(name1) name: type = expression;
    let pub(name1, name2) name3, name4: type = expression;
    */
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
    /*
    functions:

    fn name() -> type {/* block */}
    fn name() -> type = expression;
    fn name(param_1: type, param_2: type) -> type {}
    fn name(self, param_1: type) -> type {}
    fn name(mut self, param_1: type1) -> type {}
    fn async name() -> type {}
    fn pub name() -> type {}
    fn pub async name() -> type {}
    */
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
    /*
        if expression {/* body */}
        if expression {} else {}
        if expression {} else if expression {} else {}
    */
    If {
        cond: Expression,
        body: Box<Statement>,
        else_if_branches: Vec<(Vec<Expression>, Box<Statement>)>,
        else_branch: Option<Box<Statement>>,
    },
    /*
        return expression;
    */
    Return {
        expr: Expression,
    },
    /*
        while expression {/* body */}
    */
    While {
        cond: Expression,
        body: Box<Statement>,
    },
    /*
           loop {}
           loop iteration {/* */}
    */
    Loop {
        iter: Option<usize>,
        body: Box<Statement>,
    },
    /*
        break;
    */
    Break {},
    /*
        match expression {
        _ => {/* block */}
        }
        match expression {
        _ => expression
        }
        match expression {
        value1 => expr, // req comma if expression
        value2 => {}
        value3 => {}
        _ => {}
        }
    */
    Match {
        cond: Expression,
        cases: Vec<(Expression, FuncBody)>,
        def_case: FuncBody,
    },
    /*
        mod "./file.ape"
    */
    Mod {
        src: String,
    },
    /*
        use name from "./file.ape";
        use name1 as name2 from "./file.ape";
        use name1, name2, name3 from "./file.ape";
        use name1 as name2, name3 from "./file.ape";
    */
    Use {
        src: String,
        names: Vec<(Token, Option<Token>)>,
    },
    /*
        struct name {
            name1: value,
            pub name2: value,
        }
        struct pub name {}
    */
    Struct {
        name: Token,
        // (identifier, type token, is_public)
        structs: Vec<(Token, Token, bool)>,
        is_pub: bool,
    },
    /*
        impl name { /* body */ }
    */
    Impl {
        name: Token,
        // functions-only
        body: Vec<Statement>,
    },
    /*
        enum name {
            Name1,
            Name2,
            Name3,
        }
        enum pub name {}
    */
    Enum {
        // identifier
        name: Token,
        // identifiers (capitilized)
        enums: Vec<Token>,
        is_pub: bool,
    },
}

#[derive(Debug, PartialEq)]
pub enum FuncBody {
    Statements(Vec<Statement>),
    Expression(Box<Expression>),
}
