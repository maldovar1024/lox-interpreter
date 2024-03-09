use crate::{
    ast::expr::{p, BinaryOp, Expr, UnaryOp, Value},
    error::{PResult, ParserError},
    lexer::Lexer,
    precedence::Operator,
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
            $token_type => {}
            t => return Err(p(ParserError::UnexpectedToken(t, next_token.position))),
        }
    }};
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer, token: None }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
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

    fn expression(&mut self) -> PResult<Expr> {
        self.expr_precedence(Operator::None)
    }

    fn expr_precedence(&mut self, op: Operator) -> PResult<Expr> {
        let next_token = self.next_token();

        let mut expr = match next_token.token_type {
            TokenType::Keyword(kw) => Expr::Literal(match kw {
                Keyword::False => Value::Bool(true),
                Keyword::Nil => Value::Nil,
                Keyword::True => Value::Bool(true),
                _ => todo!(),
            }),
            TokenType::LeftParen => {
                let expr = Expr::group(self.expression()?);
                eat!(self, TokenType::RightParen);
                expr
            }
            TokenType::Literal(lit) => Expr::Literal(match lit {
                Literal::String(s) => Value::String(s),
                Literal::Number(n) => Value::Number(n),
            }),
            TokenType::Bang => Expr::unary(UnaryOp::Not, self.expr_precedence(Operator::Prefix)?),
            TokenType::Minus => {
                Expr::unary(UnaryOp::Negative, self.expr_precedence(Operator::Prefix)?)
            }
            t => {
                return Err(p(ParserError::ExpectStructure {
                    expected: "expression",
                    pos: next_token.position,
                    found: t,
                }))
            }
        };

        loop {
            match Operator::from_token(&self.look_ahead()) {
                Some(next_op) if next_op.is_precedent_than(op) => {
                    expr = Expr::binary(
                        BinaryOp::from_token(self.next_token())?,
                        expr,
                        self.expr_precedence(next_op)?,
                    );
                }
                _ => break,
            }
        }

        Ok(expr)
    }
}
