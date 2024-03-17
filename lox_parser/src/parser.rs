use std::mem;

use crate::{
    ast::{
        expr::{p, Expr, ExprInner, FnCall, Lit},
        stmt::{Block, Expression, FnDecl, If, Print, Statement, VarDecl, While},
    },
    error::{PResult, ParserError},
    lexer::Lexer,
    precedence::Operator,
    span::Span,
    token::{Keyword, Literal, Token, TokenType},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    token: Option<Token>,
    errors: Vec<ParserError>,
}

macro_rules! eat {
    ($self: expr, $token_type: pat) => {{
        let next_token = $self.next_token();
        match next_token.token_type {
            $token_type => next_token.span,
            t => return Err(p(ParserError::UnexpectedToken(t, next_token.span))),
        }
    }};
}

macro_rules! match_keyword {
    ($self: expr, $kw: pat) => {
        matches!($self.look_ahead(), TokenType::Keyword($kw))
    };
}

pub type Ast = Vec<Statement>;
pub type ParserResult = Result<Ast, Box<[ParserError]>>;

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            token: None,
            errors: vec![],
        }
    }

    pub fn parse(&mut self) -> ParserResult {
        let mut statements = vec![];
        while !matches!(self.look_ahead(), TokenType::Eof) {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    self.errors.push(*err);
                    self.synchronize();
                }
            }
        }
        if self.errors.len() > 0 {
            Err(mem::take(&mut self.errors).into_boxed_slice())
        } else {
            Ok(statements)
        }
    }

    fn bump(&mut self) {
        self.token = None;
    }

    fn next_token(&mut self) -> Token {
        match self.token.take() {
            Some(token) => token,
            None => self.lexer.next_token(),
        }
    }

    fn look_ahead(&mut self) -> &TokenType {
        &self
            .token
            .get_or_insert_with(|| self.lexer.next_token())
            .token_type
    }

    fn get_identifier(&mut self) -> PResult<String> {
        let next_token = self.next_token();
        match next_token.token_type {
            TokenType::Identifier(ident) => Ok(ident),
            t => Err(ParserError::expect_structure(
                "identifier",
                t,
                next_token.span,
            )),
        }
    }

    fn synchronize(&mut self) {
        loop {
            match self.look_ahead() {
                TokenType::Eof => return,
                TokenType::Semicolon => {
                    self.bump();
                    return;
                }
                TokenType::Keyword(kw)
                    if matches!(
                        kw,
                        Keyword::Class
                            | Keyword::For
                            | Keyword::Fun
                            | Keyword::If
                            | Keyword::Print
                            | Keyword::Return
                            | Keyword::Var
                            | Keyword::While
                    ) =>
                {
                    return
                }
                _ => self.bump(),
            }
        }
    }

    fn declaration(&mut self) -> PResult<Statement> {
        match self.look_ahead() {
            TokenType::Keyword(Keyword::Var) => self.var_decl(),
            TokenType::Keyword(Keyword::Fun) => self.function(),
            _ => self.statement(),
        }
    }

    fn var_decl(&mut self) -> PResult<Statement> {
        self.bump();
        let next_token = self.next_token();
        let ident = match next_token.token_type {
            TokenType::Identifier(ident) => ident,
            t => {
                return Err(ParserError::expect_structure(
                    "identifier",
                    t,
                    next_token.span,
                ))
            }
        };

        let initializer = if matches!(self.look_ahead(), TokenType::Equal) {
            self.bump();
            Some(self.expression()?)
        } else {
            None
        };

        eat!(self, TokenType::Semicolon);

        Ok(Statement::Var(VarDecl { ident, initializer }))
    }

    fn function(&mut self) -> PResult<Statement> {
        self.bump();
        let name = self.get_identifier()?;

        let start = eat!(self, TokenType::LeftParen);

        let mut parameters = vec![];
        if !matches!(self.look_ahead(), TokenType::RightParen) {
            loop {
                parameters.push(self.get_identifier()?);
                match self.look_ahead() {
                    TokenType::Comma => self.bump(),
                    _ => break,
                }
            }
        }
        let end = eat!(self, TokenType::RightParen);

        if parameters.len() > 255 {
            self.errors
                .push(ParserError::TooManyParameters(start.extends_with(&end)));
        }

        Ok(Statement::FnDecl(FnDecl {
            name,
            params: parameters.into_boxed_slice(),
            body: self.block()?,
        }))
    }

    fn statement(&mut self) -> PResult<Statement> {
        match self.look_ahead() {
            TokenType::Keyword(Keyword::Print) => self.print_statement(),
            TokenType::LeftBrace => Ok(Statement::Block(Block {
                statements: self.block()?,
            })),
            TokenType::Keyword(Keyword::If) => self.if_statement(),
            TokenType::Keyword(Keyword::While) => self.while_statement(),
            TokenType::Keyword(Keyword::For) => self.for_statement(),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> PResult<Statement> {
        self.bump();
        let stmt = Statement::Print(Print {
            expr: self.expression()?,
        });
        eat!(self, TokenType::Semicolon);
        Ok(stmt)
    }

    fn if_statement(&mut self) -> PResult<Statement> {
        self.bump();
        eat!(self, TokenType::LeftParen);
        let condition = self.expression()?;
        eat!(self, TokenType::RightParen);
        let then_branch = Box::new(self.statement()?);
        let else_branch = if match_keyword!(self, Keyword::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Statement::If(If {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn while_statement(&mut self) -> PResult<Statement> {
        self.bump();
        eat!(self, TokenType::LeftParen);
        let condition = self.expression()?;
        eat!(self, TokenType::RightParen);
        let body = Box::new(self.statement()?);
        Ok(Statement::While(While { condition, body }))
    }

    fn for_statement(&mut self) -> PResult<Statement> {
        self.bump();
        eat!(self, TokenType::LeftParen);
        let initializer = match self.look_ahead() {
            TokenType::Semicolon => {
                self.bump();
                None
            }
            TokenType::Keyword(Keyword::Var) => Some(self.var_decl()?),
            _ => Some(self.expression_statement()?),
        };

        let condition = match self.look_ahead() {
            TokenType::Semicolon => None,
            _ => Some(self.expression()?),
        };
        let condition_span = eat!(self, TokenType::Semicolon);

        let increment = match self.look_ahead() {
            TokenType::RightParen => None,
            _ => Some(self.expression()?),
        };
        eat!(self, TokenType::RightParen);

        let body = self.statement()?;

        let inner = Statement::While(While {
            condition: condition.unwrap_or(Expr {
                expr: ExprInner::Literal(Lit::Bool(true)),
                span: condition_span,
            }),
            body: match increment {
                Some(increment) => Box::new(Statement::Block(Block {
                    statements: Box::new([
                        body,
                        Statement::Expression(Expression { expr: increment }),
                    ]),
                })),
                None => Box::new(body),
            },
        });

        Ok(match initializer {
            Some(initializer) => Statement::Block(Block {
                statements: Box::new([initializer, inner]),
            }),
            None => inner,
        })
    }

    fn expression_statement(&mut self) -> PResult<Statement> {
        let stmt = Statement::Expression(Expression {
            expr: self.expression()?,
        });
        eat!(self, TokenType::Semicolon);
        Ok(stmt)
    }

    fn block(&mut self) -> PResult<Box<[Statement]>> {
        self.bump();
        let mut statements = vec![];
        while !matches!(self.look_ahead(), TokenType::RightBrace) {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    self.errors.push(*err);
                    self.synchronize();
                }
            }
        }

        eat!(self, TokenType::RightBrace);

        Ok(statements.into_boxed_slice())
    }

    fn expression(&mut self) -> PResult<Expr> {
        self.expr_precedence(Operator::None)
    }

    fn expr_precedence(&mut self, op: Operator) -> PResult<Expr> {
        let next_token = self.next_token();

        let mut expr = match next_token.token_type {
            TokenType::Keyword(kw) => Expr::literal(
                match kw {
                    Keyword::False => Lit::Bool(false),
                    Keyword::Nil => Lit::Nil,
                    Keyword::True => Lit::Bool(true),
                    _ => todo!(),
                },
                next_token.span,
            ),
            TokenType::LeftParen => {
                let grouped = self.expression()?;
                let Span { end, .. } = eat!(self, TokenType::RightParen);
                Expr::group(grouped, next_token.span.start, end)
            }
            TokenType::Literal(lit) => Expr::literal(
                match lit {
                    Literal::String(s) => Lit::String(s),
                    Literal::Number(n) => Lit::Number(n),
                },
                next_token.span,
            ),
            token_type @ (TokenType::Bang | TokenType::Minus) => Expr::unary(
                token_type.into(),
                next_token.span,
                self.expr_precedence(Operator::Prefix)?,
            ),
            TokenType::Identifier(ident) => Expr::var(ident, next_token.span),
            t => {
                return Err(p(ParserError::ExpectStructure {
                    expected: "expression",
                    found: t,
                    span: next_token.span,
                }))
            }
        };

        loop {
            match Operator::from_token(&self.look_ahead()) {
                Some(next_op) if next_op.is_precedent_than(op) => {
                    expr = match next_op {
                        Operator::Ternary => {
                            self.bump();
                            let truthy = self.expression()?;
                            eat!(self, TokenType::Colon);
                            Expr::ternary(expr, truthy, self.expr_precedence(next_op)?)
                        }
                        Operator::FnCall => self.fn_call(expr)?,
                        _ => Expr::binary(
                            self.next_token().token_type.into(),
                            expr,
                            self.expr_precedence(next_op)?,
                        ),
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn fn_call(&mut self, callee: Expr) -> PResult<Expr> {
        eat!(self, TokenType::LeftParen);
        let mut arguments = vec![];

        if !matches!(self.look_ahead(), TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);
                match self.look_ahead() {
                    TokenType::Comma => self.bump(),
                    _ => break,
                }
            }
        }

        let end = eat!(self, TokenType::RightParen);
        let span = callee.span.extends_with(&end);
        Ok(Expr {
            expr: ExprInner::FnCall(FnCall {
                callee: Box::new(callee),
                arguments: arguments.into_boxed_slice(),
            }),
            span,
        })
    }
}
