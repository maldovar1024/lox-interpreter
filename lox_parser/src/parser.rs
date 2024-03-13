use crate::{
    ast::{
        expr::{p, Expr, Value},
        stmt::{Block, Expression, Print, Statement, VarDecl},
    },
    error::{PResult, ParserError},
    lexer::Lexer,
    precedence::Operator,
    span::Span,
    token::{Keyword, Literal, Token, TokenType},
};

pub struct Parser<'a> {
    pub(crate) lexer: Lexer<'a>,
    pub(crate) token: Option<Token>,
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

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer, token: None }
    }

    pub fn parse(&mut self) -> PResult<Ast> {
        let mut statements = vec![];
        while !matches!(self.look_ahead(), TokenType::Eof) {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    pub(crate) fn bump(&mut self) {
        self.token = None;
    }

    pub(crate) fn next_token(&mut self) -> Token {
        match self.token.take() {
            Some(token) => token,
            None => self.lexer.next_token(),
        }
    }

    pub(crate) fn look_ahead(&mut self) -> &TokenType {
        &self
            .token
            .get_or_insert_with(|| self.lexer.next_token())
            .token_type
    }

    pub(crate) fn synchronize(&mut self) {
        loop {
            match self.look_ahead() {
                TokenType::Eof => return,
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
        if match_keyword!(self, Keyword::Var) {
            return self.var_decl();
        }
        self.statement()
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

    fn statement(&mut self) -> PResult<Statement> {
        match self.look_ahead() {
            TokenType::Keyword(Keyword::Print) => self.print_statement(),
            TokenType::LeftBrace => self.block(),
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

    fn expression_statement(&mut self) -> PResult<Statement> {
        let stmt = Statement::Expression(Expression {
            expr: self.expression()?,
        });
        eat!(self, TokenType::Semicolon);
        Ok(stmt)
    }

    fn block(&mut self) -> PResult<Statement> {
        self.bump();
        let mut statements = vec![];
        while !matches!(self.look_ahead(), TokenType::RightBrace) {
            statements.push(self.declaration()?);
        }

        eat!(self, TokenType::RightBrace);

        Ok(Statement::Block(Block {
            statements: statements.into_boxed_slice(),
        }))
    }

    fn expression(&mut self) -> PResult<Expr> {
        self.expr_precedence(Operator::None)
    }

    fn expr_precedence(&mut self, op: Operator) -> PResult<Expr> {
        let next_token = self.next_token();

        let mut expr = match next_token.token_type {
            TokenType::Keyword(kw) => Expr::literal(
                match kw {
                    Keyword::False => Value::Bool(false),
                    Keyword::Nil => Value::Nil,
                    Keyword::True => Value::Bool(true),
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
                    Literal::String(s) => Value::String(s),
                    Literal::Number(n) => Value::Number(n),
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
                    if matches!(next_op, Operator::Ternary) {
                        self.bump();
                        let truthy = self.expression()?;
                        eat!(self, TokenType::Colon);
                        expr = Expr::ternary(expr, truthy, self.expr_precedence(next_op)?)
                    } else {
                        expr = Expr::binary(
                            self.next_token().token_type.into(),
                            expr,
                            self.expr_precedence(next_op)?,
                        )
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }
}
