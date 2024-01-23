// use std::collections::HashMap;

use crate::Op;
use crate::Token;

type Expr = Vec<Op>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current)?.clone();
        self.current += 1;
        Some(token)
    }

    fn peek_precedence(&self) -> u8 {
        match self.tokens.get(self.current) {
            Some(token) => token.precedence(),
            None => 0,
        }
    }

    pub fn parse_expression(&mut self, precedence: u8) -> Option<Expr> {
        let token = self.consume()?;
        let prefix_parselet = token.prefix_parselets();
        let mut left = prefix_parselet(self)?;
        while precedence < self.peek_precedence() {
            let token = self.consume()?;
            let infix_parselet = token.infix_parselets();
            let mut right = infix_parselet(self, &mut left)?;
            left.append(&mut right);
        }
        Some(left)
    }
}

impl Token {
    fn precedence(&self) -> u8 {
        match self {
            Token::Plus => 2,
            Token::Minus => 2,
            Token::Star => 3,
            Token::Slash => 3,
            Token::LParen => 1,
            Token::RParen => 1,
            Token::Number(_) => 0,
        }
    }

    fn prefix_parselets(self) -> Box<dyn Fn(&mut Parser) -> Option<Expr>> {
        match self {
            Token::Number(n) => Box::new(move |_| {
                let expr = vec![Op::Push(n)];
                Some(expr)
            }),
            Token::Plus => Box::new(move |parser| {
                let expr = parser.parse_expression(self.precedence())?;
                Some(expr)
            }),
            Token::Minus => Box::new(move |parser| {
                let mut expr = parser.parse_expression(self.precedence())?;
                expr.push(Op::Negate);
                Some(expr)
            }),
            Token::LParen => Box::new(move |parser| {
                let expr = parser.parse_expression(self.precedence())?;
                Some(expr)
            }),
            _ => Box::new(move |_| None),
        }
    }

    fn infix_parselets(self) -> Box<dyn Fn(&mut Parser, &mut Expr) -> Option<Expr>> {
        match self {
            Token::Plus => Box::new(move |parser, code| {
                let mut expr = parser.parse_expression(self.precedence())?;
                code.append(&mut expr);
                code.push(Op::Add);
                Some(expr)
            }),
            Token::Minus => Box::new(move |parser, code| {
                let mut expr = parser.parse_expression(self.precedence())?;
                code.append(&mut expr);
                expr.push(Op::Sub);
                Some(expr)
            }),
            Token::Star => Box::new(move |parser, code| {
                let mut expr = parser.parse_expression(self.precedence())?;
                code.append(&mut expr);
                code.push(Op::Mul);
                Some(expr)
            }),
            Token::Slash => Box::new(move |parser, code| {
                let mut expr = parser.parse_expression(self.precedence())?;
                code.append(&mut expr);
                expr.push(Op::Div);
                Some(expr)
            }),
            Token::RParen => Box::new(move |_, _| Some(vec![])),
            _ => Box::new(move |_, _| None),
        }
    }
}
