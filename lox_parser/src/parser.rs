use crate::{
    error::{PResult, ParserError},
    precedence::Operator,
};
use lox_ast::*;
use lox_lexer::{Keyword, Lexer, Literal, Span, Token, TokenType};
use std::mem;

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
            t => return Err(Box::new(ParserError::UnexpectedToken(t, next_token.span))),
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
        if !self.errors.is_empty() {
            Err(mem::take(&mut self.errors).into_boxed_slice())
        } else {
            Ok(statements)
        }
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

    fn get_identifier(&mut self) -> PResult<Ident> {
        let next_token = self.next_token();
        match next_token.token_type {
            TokenType::Identifier(name) => Ok(Ident::from_name(name, next_token.span)),
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
                    self.next_token();
                    return;
                }
                TokenType::Keyword(
                    Keyword::Class
                    | Keyword::For
                    | Keyword::Fun
                    | Keyword::If
                    | Keyword::Print
                    | Keyword::Return
                    | Keyword::Var
                    | Keyword::While,
                ) => return,
                _ => {
                    self.next_token();
                }
            }
        }
    }

    fn declaration(&mut self) -> PResult<Statement> {
        match self.look_ahead() {
            TokenType::Keyword(Keyword::Var) => self.var_decl(),
            TokenType::Keyword(Keyword::Fun) => {
                self.next_token();
                Ok(Statement::FnDecl(Box::new(self.function()?)))
            }
            TokenType::Keyword(Keyword::Class) => self.class(),
            _ => self.statement(),
        }
    }

    fn var_decl(&mut self) -> PResult<Statement> {
        self.next_token();
        let next_token = self.next_token();
        let name = match next_token.token_type {
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
            self.next_token();
            Some(self.expression()?)
        } else {
            None
        };

        eat!(self, TokenType::Semicolon);

        Ok(Statement::Var(Box::new(VarDecl {
            var: Variable::from_name(name, next_token.span),
            initializer,
        })))
    }

    fn function(&mut self) -> PResult<FnDecl> {
        let ident = self.get_identifier()?;

        let start = eat!(self, TokenType::LeftParen);

        let mut parameters = vec![];
        if !matches!(self.look_ahead(), TokenType::RightParen) {
            loop {
                parameters.push(self.get_identifier()?.into());
                match self.look_ahead() {
                    TokenType::Comma => {
                        self.next_token();
                    }
                    _ => break,
                }
            }
        }
        let end = eat!(self, TokenType::RightParen);

        if parameters.len() > 255 {
            self.errors
                .push(ParserError::TooManyParameters(start.extends_with(&end)));
        }

        Ok(FnDecl {
            var: ident.into(),
            params: parameters.into_boxed_slice(),
            body: self.block()?,
            num_of_locals: 0,
        })
    }

    fn class(&mut self) -> PResult<Statement> {
        self.next_token();
        let ident = self.get_identifier()?;

        let super_class = if matches!(self.look_ahead(), TokenType::Less) {
            self.next_token();
            Some(self.get_identifier()?)
        } else {
            None
        };

        eat!(self, TokenType::LeftBrace);
        let mut methods = vec![];
        while !matches!(self.look_ahead(), TokenType::RightBrace) {
            methods.push(self.function()?);
        }
        eat!(self, TokenType::RightBrace);

        Ok(Statement::ClassDecl(Box::new(ClassDecl {
            var: ident.into(),
            super_class: super_class.map(From::from),
            methods: methods.into_boxed_slice(),
        })))
    }

    fn statement(&mut self) -> PResult<Statement> {
        match self.look_ahead() {
            TokenType::Keyword(Keyword::Print) => self.print_statement(),
            TokenType::LeftBrace => Ok(Statement::Block(Box::new(Block::new(self.block()?)))),
            TokenType::Keyword(Keyword::If) => self.if_statement(),
            TokenType::Keyword(Keyword::While) => self.while_statement(),
            TokenType::Keyword(Keyword::For) => self.for_statement(),
            TokenType::Keyword(Keyword::Return) => self.return_statement(),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> PResult<Statement> {
        self.next_token();
        let stmt = Statement::Print(Print {
            expr: self.expression()?,
        });
        eat!(self, TokenType::Semicolon);
        Ok(stmt)
    }

    fn if_statement(&mut self) -> PResult<Statement> {
        self.next_token();
        eat!(self, TokenType::LeftParen);
        let condition = self.expression()?;
        eat!(self, TokenType::RightParen);
        let then_branch = self.statement()?;
        let else_branch = if match_keyword!(self, Keyword::Else) {
            Some(self.statement()?)
        } else {
            None
        };
        Ok(Statement::If(Box::new(If {
            condition,
            then_branch,
            else_branch,
        })))
    }

    fn while_statement(&mut self) -> PResult<Statement> {
        self.next_token();
        eat!(self, TokenType::LeftParen);
        let condition = self.expression()?;
        eat!(self, TokenType::RightParen);
        let body = self.statement()?;
        Ok(Statement::While(Box::new(While { condition, body })))
    }

    fn for_statement(&mut self) -> PResult<Statement> {
        self.next_token();
        eat!(self, TokenType::LeftParen);
        let initializer = match self.look_ahead() {
            TokenType::Semicolon => {
                self.next_token();
                None
            }
            TokenType::Keyword(Keyword::Var) => Some(self.var_decl()?),
            _ => Some(self.expression_statement()?),
        };

        let condition = match self.look_ahead() {
            TokenType::Semicolon => None,
            _ => Some(self.expression()?),
        };

        let increment = match self.look_ahead() {
            TokenType::RightParen => None,
            _ => Some(self.expression()?),
        };
        eat!(self, TokenType::RightParen);

        let body = self.statement()?;

        let inner = Statement::While(Box::new(While {
            condition: condition.unwrap_or(Expr::literal(Lit::Bool(true), Span::dummy())),
            body: match increment {
                Some(increment) => Statement::Block(Box::new(Block::new(
                    [body, Statement::Expression(Expression { expr: increment })].into(),
                ))),
                None => body,
            },
        }));

        Ok(match initializer {
            Some(initializer) => {
                Statement::Block(Box::new(Block::new([initializer, inner].into())))
            }
            None => inner,
        })
    }

    fn return_statement(&mut self) -> PResult<Statement> {
        let token = self.next_token();
        let expr = if !matches!(self.look_ahead(), TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        eat!(self, TokenType::Semicolon);

        Ok(Statement::Return(Box::new(Return {
            span: token.span,
            expr,
        })))
    }

    fn expression_statement(&mut self) -> PResult<Statement> {
        let stmt = Statement::Expression(Expression {
            expr: self.expression()?,
        });
        eat!(self, TokenType::Semicolon);
        Ok(stmt)
    }

    fn block(&mut self) -> PResult<Box<[Statement]>> {
        self.next_token();
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
            TokenType::Keyword(kw) => match kw {
                Keyword::False => Expr::literal(Lit::Bool(false), next_token.span),
                Keyword::True => Expr::literal(Lit::Bool(true), next_token.span),
                Keyword::Nil => Expr::literal(Lit::Nil, next_token.span),
                Keyword::This => Expr::Var(Box::new(Variable::from_name(
                    "this".to_string(),
                    next_token.span,
                ))),
                Keyword::Super => Expr::Super(Box::new(Super {
                    var: Variable::from_name("super".to_string(), next_token.span),
                    method: {
                        eat!(self, TokenType::Dot);
                        self.get_identifier()?
                    },
                })),
                kw => {
                    return Err(Box::new(ParserError::UnexpectedToken(
                        TokenType::Keyword(kw),
                        next_token.span,
                    )))
                }
            },
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
            TokenType::Identifier(name) => {
                Expr::Var(Box::new(Variable::from_name(name, next_token.span)))
            }
            t => {
                return Err(Box::new(ParserError::ExpectStructure {
                    expected: "expression",
                    found: t,
                    span: next_token.span,
                }))
            }
        };

        loop {
            match Operator::from_token(self.look_ahead()) {
                Some(next_op) if next_op.is_precedent_than(op) => {
                    let next_token = self.next_token();
                    expr = match next_op {
                        Operator::Ternary => {
                            let truthy = self.expression()?;
                            eat!(self, TokenType::Colon);
                            Expr::ternary(expr, truthy, self.expr_precedence(next_op)?)
                        }
                        Operator::Assign => match expr {
                            Expr::Var(ident) => {
                                Expr::assign(*ident, self.expr_precedence(next_op)?)
                            }
                            Expr::Get(get) => Expr::set(*get, self.expr_precedence(next_op)?),
                            _ => {
                                return Err(Box::new(ParserError::InvalidLeftValue(
                                    expr.get_span(),
                                )))
                            }
                        },
                        Operator::FnCall => self.fn_call(expr)?,
                        Operator::Dot => Expr::get(expr, self.get_identifier()?),
                        _ => Expr::binary(
                            next_token.token_type.into(),
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
        let mut arguments = vec![];

        if !matches!(self.look_ahead(), TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);
                match self.look_ahead() {
                    TokenType::Comma => {
                        self.next_token();
                    }
                    _ => break,
                }
            }
        }

        let Span { end, .. } = eat!(self, TokenType::RightParen);
        Ok(Expr::FnCall(Box::new(FnCall {
            callee,
            arguments: arguments.into_boxed_slice(),
            end,
        })))
    }
}
