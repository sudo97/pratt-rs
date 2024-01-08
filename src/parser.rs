// use std::collections::HashMap;

use crate::Op;
use crate::Token;

type Expr = Vec<Op>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    // prefix_parselets: HashMap<Token, fn(&mut Parser) -> Option<Expr>>,
    // infix_parselets: HashMap<Token, fn(&mut Parser, Expr) -> Option<Expr>>,
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
            Token::Plus => 1,
            Token::Minus => 1,
            Token::Star => 2,
            Token::Slash => 2,
            _ => 0,
        }
    }

    fn prefix_parselets(self) -> Box<dyn Fn(&mut Parser) -> Option<Expr>> {
        match self {
            Token::Number(n) => Box::new(move |_| Some(vec![Op::Push(n)])),
            Token::Plus => Box::new(move |parser| {
                let expr = parser.parse_expression(self.precedence())?;
                Some(expr)
            }),
            Token::Minus => Box::new(move |parser| {
                let mut expr = parser.parse_expression(self.precedence())?;
                expr.push(Op::Negate);
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
            _ => Box::new(move |_, code| Some(code.to_vec())),
        }
    }
}
