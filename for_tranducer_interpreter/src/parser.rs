use crate::lexer::Token;
use crate::ast::{Stmt, Expr};

// Define the parser struct
pub struct Parser {
    tokens: Vec<Token>, // Vector of tokens
    current: usize,     // Current position in the token vector
}

impl Parser {
    // Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // Parse the tokens into a vector of statements (AST)
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current < self.tokens.len() {
            stmts.push(self.statement());
        }
        stmts
    }

    // Parse a single statement
    fn statement(&mut self) -> Stmt {
        match self.tokens[self.current] {
            Token::Print => {
                self.current += 1; // Consume 'print'
                self.expect(Token::LeftParen); // Expect '('
                let expr = self.expression(); // Parse the expression
                self.expect(Token::RightParen); // Expect ')'
                Stmt::Print(expr) // Return the print statement
            }
            Token::For => {
                self.current += 1; // Consume 'for'
                let var = match &self.tokens[self.current] {
                    Token::Identifier(name) => name.clone(), // Parse the variable name
                    _ => panic!("Expected identifier after 'for'"),
                };
                self.current += 1; // Consume the identifier
                self.expect(Token::In); // Expect 'in'
                let start = self.expect_number(); // Parse the start value
                self.expect(Token::DotDot); // Expect '..'
                let end = self.expect_number(); // Parse the end value
                self.expect(Token::LeftBrace); // Expect '{'
                let mut body = Vec::new(); // Vector to hold the body statements
                while !self.matches(Token::RightBrace) {
                    body.push(self.statement()); // Parse each statement in the body
                }
                self.expect(Token::RightBrace); // Expect '}'
                Stmt::For(var, start, end, body) // Return the for statement
            }
            _ => panic!("Unexpected token: {:?}", self.tokens[self.current]), // Handle unexpected tokens
        }
    }

    // Parse an expression
    fn expression(&mut self) -> Expr {
        match &self.tokens[self.current] {
            Token::Number(n) => {
                self.current += 1; // Consume the number
                Expr::Number(*n) // Return the number expression
            }
            Token::Identifier(name) => {
                self.current += 1; // Consume the identifier
                Expr::Var(name.clone()) // Return the variable expression
            }
            _ => panic!("Unexpected token in expression: {:?}", self.tokens[self.current]), // Handle unexpected tokens
        }
    }

    // Expect a specific token and advance the current position
    fn expect(&mut self, token: Token) {
        if self.tokens[self.current] == token {
            self.current += 1;
        } else {
            panic!("Expected token: {:?}, found: {:?}", token, self.tokens[self.current]);
        }
    }

    // Expect a number token and return its value
    fn expect_number(&mut self) -> i32 {
        if let Token::Number(n) = self.tokens[self.current] {
            self.current += 1;
            n
        } else {
            panic!("Expected number, found: {:?}", self.tokens[self.current]);
        }
    }

    // Check if the current token matches the given token and advance if it does
    fn matches(&mut self, token: Token) -> bool {
        if self.tokens[self.current] == token {
            self.current += 1;
            true
        } else {
            false
        }
    }
}
