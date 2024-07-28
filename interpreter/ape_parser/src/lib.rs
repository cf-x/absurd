#[cfg(test)]
mod tests;
use ape_ast::{
    CallType, Expression, FuncBody, LiteralKind, LiteralType, Statement, Token,
    TokenType::{self, *},
};
use core::panic;
use std::process::exit;

pub struct Parser {
    tokens: Vec<Token>,
    crnt: usize,
    id: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            crnt: 0,
            id: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stmts = vec![];
        while !self.check(Eof) {
            let stmt = self.stmt();
            stmts.push(stmt);
        }
        stmts
    }

    fn stmt(&mut self) -> Statement {
        self.advance();
        match self.prev(1).token {
            Let => self.var_stmt(),
            Func => self.func_stmt(),
            If => self.if_stmt(),
            Return => self.return_stmt(),
            While => self.while_stmt(),
            Loop => self.loop_stmt(),
            Break => self.break_stmt(),
            Match => self.match_stmt(),
            Mod => self.mod_stmt(),
            Use => self.use_stmt(),
            Struct => self.struct_stmt(),
            Impl => self.impl_stmt(),
            Enum => self.enum_stmt(),
            LeftBrace => self.block_stmt(),
            _ => self.expr_stmt(),
        }
    }

    fn var_stmt(&mut self) -> Statement {
        let mut names: Vec<Token> = vec![];
        let mut pub_names: Vec<Token> = vec![];
        let mut is_mut = false;
        let mut is_pub = false;
        let mut is_null = false;

        if self.if_token_consume(Mut) {
            is_mut = true;
        } else if self.if_token_consume(Pub) {
            is_pub = true;
            if self.if_token_consume(LeftParen) {
                loop {
                    let name = self.consume(Ident);
                    pub_names.push(name);
                    if !self.is_token(Comma) || self.is_token(RightParen) {
                        break;
                    }
                    self.advance();
                }
                self.consume(RightParen);
            }
        }

        loop {
            let name = self.consume(Ident);
            names.push(name);

            if self.is_token(Semi) {
                is_null = true;
                break;
            }

            if !self.is_token(Comma) || self.is_token(Colon) {
                break;
            }
            self.advance();
        }

        let null_var = Statement::Var {
            names: names.clone(),
            value_type: Token {
                token: NullIdent,
                len: 4,
                lexeme: "null".to_string(),
                value: None,
                line: names[0].line,
            },
            value: Some(Expression::Value {
                id: self.id,
                value: LiteralType::Null,
            }),
            is_mut,
            is_pub,
            pub_names: pub_names.clone(),
            is_func: false,
        };

        if is_null {
            self.advance();
            return null_var;
        }

        self.consume(Colon);
        let value_type = self.consume_type_ident();
        self.advance();

        if value_type.token == NullIdent {
            return null_var;
        }

        self.consume(Assign);
        let is_func = self.is_token(Pipe);
        let value = self.expr();
        self.consume(Semi);

        Statement::Var {
            names,
            value_type,
            value: Some(value),
            is_mut,
            is_pub,
            pub_names,
            is_func,
        }
    }

    fn func_stmt(&mut self) -> Statement {
        let mut params: Vec<(Token, Token)> = vec![];
        let mut is_async = false;
        let mut is_pub = false;
        let mut is_impl = false;
        let mut is_mut = false;

        if self.if_token_consume(Pub) {
            is_pub = true;
            if self.if_token_consume(Async) {
                is_async = true;
            }
        }

        if self.if_token_consume(Async) {
            is_async = true;
            if self.if_token_consume(Pub) {
                is_pub = true;
            }
        }

        let name = self.consume(Ident);

        self.consume(LeftParen);
        while !self.is_token(RightParen) {
            if self.is_token(Ident) {
                let param_name = self.consume(Ident);
                self.consume(Colon);
                let param_type = self.consume_type_ident();
                params.push((param_name, param_type))
            } else if self.if_token_consume(Mut) {
                self.consume(Slf);
                is_mut = true;
                is_impl = true;
            } else if self.if_token_consume(Slf) {
                is_impl = true;
            } else if self.if_token_consume(Comma) {
            } else if !self.is_token(RightParen) {
                // @error unexpected token
                panic!("@error unexpected token '{}'", self.prev(1).lexeme);
            }
        }
        self.consume(RightParen);
        self.consume(Arrow);
        let value_type = self.consume_type_ident();

        if self.if_token_consume(Assign) {
            let body = self.expr();
            self.consume(Semi);
            return Statement::Func {
                name,
                value_type,
                body: FuncBody::Statements(vec![Statement::Return { expr: body }]),
                params,
                is_async,
                is_pub,
                is_impl,
                is_mut,
            };
        }

        self.consume(LeftBrace);
        let body = self.block_stmts();

        Statement::Func {
            name,
            value_type,
            body: FuncBody::Statements(body),
            params,
            is_async,
            is_pub,
            is_impl,
            is_mut,
        }
    }

    fn if_stmt(&mut self) -> Statement {
        let cond = self.expr();
        let body = self.block_stmts();
        let mut else_if_branches = vec![];

        while self.if_token_consume(ElseIf) {
            // @todo remove Vec from ElseIf pred type
            let elif_preds = vec![self.expr()];
            let elif_stmt = self.block_stmts();
            else_if_branches.push((elif_preds, elif_stmt))
        }

        let else_branch = if self.if_token_consume(Else) {
            Some(self.block_stmts())
        } else {
            None
        };

        Statement::If {
            cond,
            body,
            else_if_branches,
            else_branch,
        }
    }

    fn return_stmt(&mut self) -> Statement {
        let expr;
        if self.is_token(Semi) {
            expr = Expression::Value {
                id: self.id(),
                value: LiteralType::Null,
            }
        } else {
            expr = self.expr()
        }
        self.consume(Semi);
        Statement::Return { expr }
    }

    fn while_stmt(&mut self) -> Statement {
        let cond = self.expr();
        let body = self.block_stmts();
        Statement::While { cond, body }
    }

    fn loop_stmt(&mut self) -> Statement {
        let iter = if self.if_token_consume(NumberLit) {
            let num = match self.consume(NullLit).value {
                Some(LiteralKind::Number { value, .. }) => value,
                _ => {
                    panic!("@error failed to unwrap a number");
                }
            };
            Some(num as usize)
        } else {
            None
        };

        let body = self.block_stmts();
        Statement::Loop { iter, body }
    }

    fn break_stmt(&mut self) -> Statement {
        self.consume(Semi);
        Statement::Break {}
    }

    fn match_stmt(&mut self) -> Statement {
        let cond = self.expr();
        self.consume(LeftBrace);
        let mut cases = vec![];

        while self.is_literal() || self.is_token(Ident) {
            let expr = self.expr();
            self.consume(ArrowBig);
            if self.if_token_advance(LeftBrace) {
                let body = self.block_stmts();
                self.consume(RightBrace);
                cases.push((expr, FuncBody::Statements(body)))
            } else {
                let body = self.expr();
                self.consume(Comma);
                cases.push((expr, FuncBody::Expression(Box::new(body))))
            }
        }

        self.consume(Underscore);
        self.consume(ArrowBig);

        let stmt = if self.if_token_consume(LeftBrace) {
            let body = self.block_stmts();
            Statement::Match {
                cond,
                cases,
                def_case: FuncBody::Statements(body),
            }
        } else {
            let body = self.expr();
            self.consume(Comma);
            Statement::Match {
                cond,
                cases,
                def_case: FuncBody::Expression(Box::new(body)),
            }
        };
        self.consume(RightBrace);
        stmt
    }

    fn mod_stmt(&mut self) -> Statement {
        let src = self.consume(StringLit).lexeme;
        self.consume(Semi);
        Statement::Mod { src }
    }

    fn use_stmt(&mut self) -> Statement {
        let mut names: Vec<(Token, Option<Token>)> = vec![];
        while !self.if_token_advance(From) {
            let name = self.consume(Ident);
            if self.if_token_consume(As) {
                let as_name = self.consume(Ident);
                names.push((name, Some(as_name)))
            } else {
                names.push((name, None))
            }
            self.consume(Comma);
        }

        let src = self.consume(StringLit).lexeme;
        self.consume(Semi);
        Statement::Use { src, names }
    }

    // @todo handle uppercase names
    fn struct_stmt(&mut self) -> Statement {
        let mut is_pub = false;
        if self.if_token_consume(Pub) {
            is_pub = true;
        }

        let name = self.consume(Ident);
        self.consume(LeftBrace);
        let mut structs: Vec<(Token, TokenType, bool)> = vec![];
        while !self.if_token_consume(RightBrace) {
            let mut struct_is_pub = false;
            if self.if_token_consume(Pub) {
                struct_is_pub = true;
            }

            let struct_name = self.consume(Ident);
            self.consume(Colon);
            let struct_type = self.consume_type_ident().token;
            structs.push((struct_name, struct_type, struct_is_pub));

            if !self.if_token_consume(Comma) && !self.is_token(RightBrace) {
                // @error unexpected token
                panic!("@error unexpected token '{}'", self.peek().lexeme);
            }
        }
        Statement::Struct {
            name,
            structs,
            is_pub,
            methods: vec![],
        }
    }

    fn impl_stmt(&mut self) -> Statement {
        let name = self.consume(Ident);
        self.consume(LeftBrace);
        let mut body: Vec<Statement> = vec![];
        while !self.if_token_consume(RightBrace) && !self.is_token(Eof) {
            self.advance();
            let func = self.func_stmt();
            body.push(func);
        }

        Statement::Impl { name, body }
    }

    fn enum_stmt(&mut self) -> Statement {
        let mut is_pub = false;

        if self.if_token_consume(Pub) {
            is_pub = true;
        }

        let name = self.consume(Ident);
        self.consume(LeftBrace);

        let mut enums: Vec<Token> = vec![];
        while !self.if_token_consume(RightBrace) {
            let enm = self.consume(Ident);
            enums.push(enm);
            if !self.if_token_consume(Comma) && !self.is_token(RightBrace) {
                // @error unexpected token
                panic!("@error unexpected token '{}'", self.peek().lexeme);
            }
        }
        Statement::Enum {
            name,
            enums,
            is_pub,
        }
    }

    fn block_stmts(&mut self) -> Vec<Statement> {
        match self.block_stmt() {
            Statement::Block { stmts } => {
                self.consume(RightBrace);
                return stmts;
            }
            _ => {
                // @error failed to parse
                panic!("@error failed to parse a block statement")
            }
        }
    }

    fn block_stmt(&mut self) -> Statement {
        let mut stmts = vec![];
        while !self.is_token(RightBrace) && !self.is_token(Eof) {
            let stmt = self.stmt();
            stmts.push(stmt);
        }
        Statement::Block { stmts }
    }

    fn expr_stmt(&mut self) -> Statement {
        let expr = self.expr();
        self.consume(Semi);
        Statement::Expression { expr }
    }

    fn expr(&mut self) -> Expression {
        self.binary()
    }

    fn binary(&mut self) -> Expression {
        let mut expr: Expression = self.unary();
        while self.are_tokens(vec![
            Plus,
            Minus,
            Mult,
            Divide,
            Percent,
            AndAnd,
            Or,
            Eq,
            NotEq,
            Greater,
            GreaterOrEq,
            Less,
            LessOrEq,
            PlusEq,
            MinEq,
            MultEq,
            DivEq,
            Square,
            And,
        ]) {
            self.advance();
            let operator = self.prev(1);
            let rhs = self.unary();
            expr = Expression::Binary {
                id: self.id(),
                left: Box::new(expr),
                operator,
                right: Box::new(rhs),
            }
        }
        expr
    }

    fn unary(&mut self) -> Expression {
        if self.are_tokens(vec![Not, NotNot, Queston, Decr, Increment]) {
            self.advance();
            let operator = self.prev(1);
            let rhs = self.unary();
            Expression::Unary {
                id: self.id(),
                left: Box::new(rhs),
                operator,
            }
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Expression {
        let mut expr = self.primary();
        // @todo add whole structure calls
        loop {
            if self.if_token_consume(Dot) {
                expr = self.struct_call();
            } else if self.if_token_consume(DblColon) {
                expr = self.enum_call();
            } else if self.if_token_consume(LeftParen) {
                expr = self.func_call();
            } else if self.if_token_consume(Ident) {
                expr = self.call();
            } else {
                break;
            }
        }
        expr
    }

    // @todo add method type
    fn struct_call(&mut self) -> Expression {
        let name = self.prev(2);
        let mut args = vec![];
        let arg = self.expr();
        args.push(arg);
        let call_type = CallType::Struct;

        Expression::Call {
            id: self.id(),
            name: Box::new(name),
            args,
            call_type,
        }
    }

    fn enum_call(&mut self) -> Expression {
        let name = self.prev(3);
        let mut args = vec![];
        let arg = self.expr();
        args.push(arg);
        let call_type = CallType::Enum;

        Expression::Call {
            id: self.id(),
            name: Box::new(name),
            args,
            call_type,
        }
    }

    fn func_call(&mut self) -> Expression {
        let name = self.prev(2);
        let mut args = vec![];
        let call_type = CallType::Func;
        while !self.is_token(RightParen) && !self.is_token(Eof) {
            let arg = self.expr();
            args.push(arg);
            if !self.is_token(Comma) && !self.is_token(RightBrace) {
                // @error unexpected token
                exit(1);
            }
            self.consume(Comma);
        }
        Expression::Call {
            id: self.id(),
            name: Box::new(name),
            args,
            call_type,
        }
    }

    fn primary(&mut self) -> Expression {
        let token = self.peek();
        match token.clone().token {
            Ident => {
                self.advance();
                let mut expr = Expression::Var {
                    id: self.id(),
                    name: self.prev(1),
                };

                if self.if_token_consume(LeftBracket) {
                    expr = self.arr_expr()
                }
                return expr;
            }
            LeftBracket => {
                self.advance();
                return self.arr_expr();
            }
            LeftParen => return self.group_expr(),
            Pipe => return self.func_expr(),
            If => return self.if_expr(),
            While => return self.while_expr(),
            Loop => return self.loop_expr(),
            Match => return self.match_expr(),
            Await => return self.await_expr(),
            _ => {
                if self.is_literal() {
                    self.advance();
                    return Expression::Value {
                        id: self.id(),
                        value: self.to_value_type(token),
                    };
                }
                // @error unexpected token token.lexeme
                panic!("@error unexpected token '{}'", token.lexeme);
            }
        }
    }

    fn to_value_type(&mut self, token: Token) -> LiteralType {
        match token.token {
            NumberLit => {
                let number = match token.value {
                    Some(LiteralKind::Number { value, .. }) => value,
                    _ => {
                        panic!("@error failed to unwrap a number");
                    }
                };

                LiteralType::Number(number)
            }
            StringLit => {
                let string = match token.value {
                    Some(LiteralKind::String { value }) => value,
                    _ => {
                        panic!("@error failed to unwrap a number");
                    }
                };
                LiteralType::String(string)
            }
            CharLit => {
                let char = match token.value {
                    Some(LiteralKind::Char { value }) => value,
                    _ => {
                        panic!("@error failed to unwrap a number");
                    }
                };
                LiteralType::Char(char)
            }
            TrueLit => LiteralType::Boolean(true),
            FalseLit => LiteralType::Boolean(false),
            NullLit => LiteralType::Null,
            _ => LiteralType::Any,
        }
    }

    fn arr_expr(&mut self) -> Expression {
        let mut items = vec![];
        while !self.is_token(RightBracket) && !self.is_token(Eof) {
            let item_expr = self.expr();
            let item = match item_expr {
                Expression::Value { value, .. } => value,
                _ => {
                    // @error expected a value expression
                    exit(1)
                }
            };
            items.push(item);
            if !self.is_token(Comma) {
                break;
            }
        }
        Expression::Array {
            id: self.id(),
            items,
        }
    }

    fn group_expr(&mut self) -> Expression {
        self.advance();
        let expr = self.expr();
        self.consume(RightParen);
        Expression::Grouping {
            id: self.id(),
            expression: Box::new(expr),
        }
    }

    // @todo add after adding callback literal types
    fn func_expr(&mut self) -> Expression {
        self.call()
    }

    // @todo add after fixing expression ast-s
    fn if_expr(&mut self) -> Expression {
        self.call()
    }
    fn while_expr(&mut self) -> Expression {
        self.call()
    }
    fn loop_expr(&mut self) -> Expression {
        self.call()
    }
    fn match_expr(&mut self) -> Expression {
        self.call()
    }
    fn await_expr(&mut self) -> Expression {
        self.call()
    }

    /// checks if current token is literal value
    fn is_literal(&self) -> bool {
        self.are_tokens(vec![
            NumberLit, StringLit, CharLit, TrueLit, FalseLit, NullLit,
        ])
    }

    /// consumes if token matches
    fn if_token_consume(&mut self, token: TokenType) -> bool {
        if self.is_token(token.clone()) {
            self.consume(token);
            return true;
        }
        false
    }

    /// advances if token matches
    fn if_token_advance(&mut self, token: TokenType) -> bool {
        if self.is_token(token) {
            self.advance();
            return true;
        }
        false
    }

    /// advances if token is type identifier
    fn consume_type_ident(&mut self) -> Token {
        self.consume_some(vec![
            AnyIdent,
            BoolIdent,
            CharIdent,
            NullIdent,
            VoidIdent,
            ArrayIdent,
            NumberIdent,
            StringIdent,
        ])
    }

    /// advances if one of the input tokens matches
    fn consume_some(&mut self, ts: Vec<TokenType>) -> Token {
        for t in ts {
            if self.if_token_advance(t) {
                return self.prev(1);
            }
        }
        let token = self.prev(1);
        // @error expected token.lexeme, at token.line
        eprintln!("expected '{}', at {}", token.lexeme, token.line);
        token
    }

    /// advances if input token matches
    fn consume(&mut self, t: TokenType) -> Token {
        if self.if_token_advance(t) {
            return self.prev(1);
        }
        let token = self.prev(1);
        // @error expected token.lexeme, at token.line
        eprintln!("expected '{}', at {}", token.lexeme, token.line);
        token
    }

    /// increases current position by 1
    /// and returns advanced token
    fn advance(&mut self) -> Token {
        if !self.is_token(Eof) {
            self.crnt += 1;
        }
        self.prev(1)
    }

    /// returns previous token
    fn prev(&self, back: usize) -> Token {
        if self.crnt < back {
            return Token {
                token: Eof,
                lexeme: "\0".to_string(),
                line: 0,
                len: 0,
                value: None,
            };
        }
        self.tokens[self.crnt - back].clone()
    }

    /// bulk checks if one of the token matches current token
    fn are_tokens(&self, tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if self.is_token(token.clone()) {
                return true;
            }
        }
        false
    }

    /// checks if token matches current token and
    /// handles EoF
    fn is_token(&self, token: TokenType) -> bool {
        if !self.check(Eof) && self.check(token) {
            return true;
        }
        false
    }

    /// checks if token matches current token
    fn check(&self, token: TokenType) -> bool {
        self.peek().token == token
    }

    /// returns current token
    fn peek(&self) -> Token {
        self.tokens[self.crnt].clone()
    }

    /// increases id count, and returns previous id
    fn id(&mut self) -> usize {
        self.id += 1;
        self.id - 1
    }
}
