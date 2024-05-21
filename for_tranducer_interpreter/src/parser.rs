use crate::lexer::Token;
use crate::ast::{Stmt, Expr};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current < self.tokens.len() {
            stmts.push(self.statement());
        }
        stmts
    }

    fn statement(&mut self) -> Stmt {
        match self.tokens[self.current] {
            Token::Print => {
                self.current += 1; // consume 'print'
                self.expect(Token::LeftParen);
                let expr = self.expression();
                self.expect(Token::RightParen);
                Stmt::Print(expr)
            }
            Token::For => {
                self.current += 1; // consume 'for'
                let var = match &self.tokens[self.current] {
                    Token::Identifier(name) => name.clone(),
                    _ => panic!("Expected identifier after 'for'"),
                };
                self.current += 1; // consume identifier
                self.expect(Token::In);
                let start = self.expect_number();
                self.expect(Token::DotDot);
                let end = self.expect_number();
                self.expect(Token::LeftBrace);
                let mut body = Vec::new();
                while !self.matches(Token::RightBrace) {
                    body.push(self.statement());
                }
                self.expect(Token::RightBrace);
                Stmt::For(var, start, end, body)
            }
            _ => panic!("Unexpected token: {:?}", self.tokens[self.current]),
        }
    }

    fn expression(&mut self) -> Expr {
        match &self.tokens[self.current] {
            Token::Number(n) => {
                self.current += 1;
                Expr::Number(*n)
            }
            Token::Identifier(name) => {
                self.current += 1;
                Expr::Var(name.clone())
            }
            _ => panic!("Unexpected token in expression: {:?}", self.tokens[self.current]),
        }
    }

    fn expect(&mut self, token: Token) {
        if self.tokens[self.current] == token {
            self.current += 1;
        } else {
            panic!("Expected token: {:?}, found: {:?}", token, self.tokens[self.current]);
        }
    }

    fn expect_number(&mut self) -> i32 {
        if let Token::Number(n) = self.tokens[self.current] {
            self.current += 1;
            n
        } else {
            panic!("Expected number, found: {:?}", self.tokens[self.current]);
        }
    }

    fn matches(&mut self, token: Token) -> bool {
        if self.tokens[self.current] == token {
            self.current += 1;
            true
        } else {
            false
        }
    }
}
