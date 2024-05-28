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
        match self.peek() {
            Some(Token::Print) => {
                self.current += 1;
                self.expect(Token::LeftParen);
                let expr = self.expression();
                self.expect(Token::RightParen);
                Stmt::Print(expr)
            }
            Some(Token::For) => {
                self.current += 1;
                let var = {
                    if let Some(Token::Identifier(name)) = self.peek().cloned() {
                        self.current += 1;
                        name
                    } else {
                        panic!("Expected identifier after 'for'");
                    }
                };
                self.expect(Token::In);
                let start = self.expect_number();
                self.expect(Token::DotDot);
                let end = self.expect_number();
                self.expect(Token::LeftBrace);
                let mut body = Vec::new();
                while !self.check(Token::RightBrace) {
                    body.push(self.statement());
                }
                self.expect(Token::RightBrace);
                Stmt::For(var, start, end, body)
            }
            Some(Token::If) => self.if_statement(),
            _ => panic!("Expected statement"),
        }
    }

    fn if_statement(&mut self) -> Stmt {
        self.expect(Token::If);
        let condition = self.expression();
        self.expect(Token::LeftBrace);
        let mut then_branch = Vec::new();
        while !self.check(Token::RightBrace) {
            then_branch.push(self.statement());
        }
        self.expect(Token::RightBrace);
        let else_branch = if self.match_token(Token::Else) {
            self.expect(Token::LeftBrace);
            let mut else_branch = Vec::new();
            while !self.check(Token::RightBrace) {
                else_branch.push(self.statement());
            }
            self.expect(Token::RightBrace);
            else_branch
        } else {
            Vec::new()
        };
        Stmt::If(condition, then_branch, else_branch)
    }

    fn expression(&mut self) -> Expr {
        let mut expr = self.term();
        while let Some(token) = self.peek().cloned() {
            match token {
                Token::LessEqual | Token::Less | Token::Equal => {
                    self.current += 1;
                    let right = Box::new(self.term());
                    expr = match token {
                        Token::LessEqual => Expr::LessEqual(Box::new(expr), right),
                        Token::Less => Expr::Less(Box::new(expr), right),
                        Token::Equal => Expr::Equal(Box::new(expr), right),
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        match self.peek().cloned() {
            Some(Token::Number(n)) => {
                self.current += 1;
                Expr::Number(n)
            }
            Some(Token::String(s)) => {
                self.current += 1;
                Expr::Str(s)
            }
            Some(Token::Identifier(name)) => {
                self.current += 1;
                Expr::Var(name)
            }
            _ => panic!("Expected expression"),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn check(&self, token: Token) -> bool {
        matches!(self.peek(), Some(t) if *t == token)
    }

    fn match_token(&mut self, token: Token) -> bool {
        if self.check(token.clone()) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, token: Token) {
        if self.check(token.clone()) {
            self.current += 1;
        } else {
            panic!("Expected token: {:?}", token);
        }
    }

    fn expect_number(&mut self) -> i32 {
        if let Some(Token::Number(n)) = self.peek().cloned() {
            self.current += 1;
            n
        } else {
            panic!("Expected number");
        }
    }
}
