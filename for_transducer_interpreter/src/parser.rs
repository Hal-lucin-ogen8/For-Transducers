use crate::lexer::Token;
use crate::ast::{Stmt, Expr};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // Create a new parser with a list of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // Parse the tokens into a vector of statements
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current < self.tokens.len() {
            stmts.push(self.statement());
        }
        stmts
    }

    // Parse a single statement
    fn statement(&mut self) -> Stmt {
        match self.peek().cloned() {
            // Parse a print statement
            Some(Token::Print) => {
                self.current += 1;
                self.expect(Token::LeftParen);
                let expr = self.expression();
                self.expect(Token::RightParen);
                Stmt::Print(expr)
            }
            // Parse a for loop
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
                let start = self.expect_start_token();
                self.expect(Token::DotDot);
                let end = self.expect_end_token();
                self.expect(Token::LeftBrace);
                let mut body = Vec::new();
                while !self.check(Token::RightBrace) {
                    body.push(self.statement());
                }
                self.expect(Token::RightBrace);

                // Validate the start and end conditions
                let direction = match (start, end) {
                    (Expr::Number(0), Expr::Var(_)) => false, // Ascending: for i in 0..n
                    (Expr::Var(_), Expr::Number(0)) => true,  // Descending: for i in n..0
                    _ => panic!("Invalid for loop syntax. Only 'for i in 0..n' or 'for i in n..0' are allowed."),
                };
                // println!("{}: {}", var, direction);
                Stmt::For(var, direction, body)
            }
            // Parse an if statement
            Some(Token::If) => self.if_statement(),
            _ => panic!("Expected statement"),
        }
    }

    // Parse an if statement
    fn if_statement(&mut self) -> Stmt {
        self.current += 1;

        let condition = self.expression();

        self.expect(Token::LeftBrace);
        let mut then_branch = Vec::new();
        while !self.check(Token::RightBrace) {
            then_branch.push(self.statement());
        }
        self.expect(Token::RightBrace);

        let mut else_branch = Vec::new();
        if let Some(Token::Else) = self.peek() {
            self.current += 1;
            self.expect(Token::LeftBrace);
            while !self.check(Token::RightBrace) {
                else_branch.push(self.statement());
            }
            self.expect(Token::RightBrace);
        }

        Stmt::If(condition, then_branch, else_branch)
    }

    // Parse an expression
    fn expression(&mut self) -> Expr {
        let mut expr = self.term();

        // Parse binary operators
        while let Some(token) = self.peek().cloned() {
            match token {
                Token::LessEqual | Token::Less | Token::Equal | Token::NotEqual => {
                    self.current += 1;
                    let right = Box::new(self.term());
                    expr = match token {
                        Token::LessEqual => Expr::LessEqual(Box::new(expr), right),
                        Token::Less => Expr::Less(Box::new(expr), right),
                        Token::Equal => Expr::Equal(Box::new(expr), right),
                        Token::NotEqual => Expr::NotEqual(Box::new(expr), right),
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }
        expr
    }

    // Parse a term (number, string, variable, or label)
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
            Some(Token::Label(name)) => {
                self.current += 1;
                Expr::Label(name)
            }
            _ => panic!("Expected expression"),
        }
    }

    // Peek at the current token without consuming it
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    // Check if the current token matches the given token
    fn check(&self, token: Token) -> bool {
        matches!(self.peek(), Some(t) if *t == token)
    }

    // Consume the current token if it matches the given token, otherwise panic
    fn expect(&mut self, token: Token) {
        if self.check(token.clone()) {
            self.current += 1;
        } else {
            panic!("Expected token: {:?}", token);
        }
    }

    // Expect a start token for a for loop ('0' or 'n')
    fn expect_start_token(&mut self) -> Expr {
        match self.peek().cloned() {
            Some(Token::Number(0)) => {
                self.current += 1;
                Expr::Number(0)
            }
            Some(Token::Identifier(name)) if name == "n" => {
                self.current += 1;
                Expr::Var(name.clone())
            }
            _ => panic!("Expected start token to be '0' or 'n'"),
        }
    }

    // Expect an end token for a for loop ('0' or 'n')
    fn expect_end_token(&mut self) -> Expr {
        match self.peek().cloned() {
            Some(Token::Number(0)) => {
                self.current += 1;
                Expr::Number(0)
            }
            Some(Token::Identifier(name)) if name == "n" => {
                self.current += 1;
                Expr::Var(name.clone())
            }
            _ => panic!("Expected end token to be '0' or 'n'"),
        }
    }
}
