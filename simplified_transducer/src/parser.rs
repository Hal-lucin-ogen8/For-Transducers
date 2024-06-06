use crate::lexer::Token;
use crate::ast::{Stmt, Bexpr, Pexpr, Fexpr};


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Bexpr {
    // Function to apply logical NOT to a boolean expression
    fn logical_not(expr: Bexpr) -> Bexpr {
        match expr {
            Bexpr::Equal(left, right) => Bexpr::NotEqual(left, right),
            Bexpr::NotEqual(left, right) => Bexpr::Equal(left, right),
            Bexpr::Less(left, right) => Bexpr::GreaterEqual(left, right),
            Bexpr::GreaterEqual(left, right) => Bexpr::Less(left, right),
            Bexpr::Greater(left, right) => Bexpr::LessEqual(left, right),
            Bexpr::LessEqual(left, right) => Bexpr::Greater(left, right),
            _ => panic!("Cannot apply logical NOT to this expression"),
        }
    }
}

impl Parser {
    // Create a new parser with a list of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // Parse the tokens into a vector of statements
    // Parse the tokens into a vector of statements
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current < self.tokens.len() {
            stmts.extend(self.statement());
        }
        stmts
    }

    // Parse a single statement
    fn statement(&mut self) -> Vec<Stmt> {
        match self.peek().cloned() {
            // Parse a print statement
            Some(Token::Print) => {
                self.current += 1;
                self.expect(Token::LeftParen);
                let expr = self.p_expression();
                self.expect(Token::RightParen);
                vec![Stmt::Print(expr)]
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
                    body.extend(self.statement());
                }
                self.expect(Token::RightBrace);

                // Validate the start and end conditions
                let direction = match (start, end) {
                    (Fexpr::Number(0), Fexpr::Var(_)) => false, // Ascending: for i in 0..n
                    (Fexpr::Var(_), Fexpr::Number(0)) => true,  // Descending: for i in n..0
                    _ => panic!("Invalid for loop syntax. Only 'for i in 0..n' or 'for i in n..0' are allowed."),
                };

                if direction {
                    vec![Stmt::For1(var, body)]
                } else {
                    vec![Stmt::For0(var, body)]
                }
            }
            // Parse an if statement
            Some(Token::If) => self.if_statement(),
            _ => panic!("Expected statement"),
        }
    }

    // Parse an if statement
    fn if_statement(&mut self) -> Vec<Stmt> {
        self.current += 1;

        let condition = self.b_expression();

        self.expect(Token::LeftBrace);
        let mut then_branch = Vec::new();
        while !self.check(Token::RightBrace) {
            then_branch.extend(self.statement());
        }
        self.expect(Token::RightBrace);

        let mut statements = vec![Stmt::If(condition.clone(), then_branch)];

        // Check if there's an else branch
        if let Some(Token::Else) = self.peek() {
            // Skip over "else" token
            self.current += 1;

            // Parse the else branch
            self.expect(Token::LeftBrace);
            let mut else_branch = Vec::new();
            while !self.check(Token::RightBrace) {
                else_branch.extend(self.statement());
            }
            self.expect(Token::RightBrace);

            // Create the negated condition for the else branch
            let negated_condition = Bexpr::logical_not(condition);

            // Create an If statement for the else branch
            let else_stmt = Stmt::If(negated_condition, else_branch);

            // Add the else statement to the list of statements
            statements.push(else_stmt);
        }

        statements
    }

    // Parse a print expression
    fn p_expression(&mut self) -> Pexpr {
        match self.peek().cloned() {
            Some(Token::String(s)) => {
                self.current += 1;
                Pexpr::Str(s)
            }

            Some(Token::Label(name)) => {
                self.current += 1;
                Pexpr::Label(name)
            }
            _ => panic!("Expected string, variable, or label"),
        }
    }

    // Parse a boolean expression
    fn b_expression(&mut self) -> Bexpr {
        let mut expr = self.term();

        // Parse binary operators
        while let Some(token) = self.peek().cloned() {
            match token {
                Token::LessEqual | Token::Less | Token::Equal | Token::NotEqual | Token::GreaterEqual | Token::Greater => {
                    self.current += 1;
                    let right = Box::new(self.term());
                    expr = match token {
                        Token::LessEqual => Bexpr::LessEqual(Box::new(expr), right),
                        Token::Less => Bexpr::Less(Box::new(expr), right),
                        Token::Equal => Bexpr::Equal(Box::new(expr), right),
                        Token::NotEqual => Bexpr::NotEqual(Box::new(expr), right),
                        Token::GreaterEqual => Bexpr::GreaterEqual(Box::new(expr), right),
                        Token::Greater => Bexpr::Greater(Box::new(expr), right),
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }
        expr
    }

    
    // Parse a term (number, string, variable, or label)
    fn term(&mut self) -> Bexpr{
        match self.peek().cloned() {
            Some(Token::String(s)) => {
                self.current += 1;
                Bexpr::Str(s)
            }
            Some(Token::Identifier(name)) => {
                self.current += 1;
                Bexpr::Var(name)
            }
            Some(Token::Label(name)) => {
                self.current += 1;
                Bexpr::Label(name)
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

    fn expect_start_token(&mut self) -> Fexpr {
        match self.peek().cloned() {
            Some(Token::Number(0)) => {
                self.current += 1;
                Fexpr::Number(0)
            }
            Some(Token::Identifier(name)) if name == "n" => {
                self.current += 1;
                Fexpr::Var(name.clone())
            }
            _ => panic!("Expected start token to be '0' or 'n'"),
        }
    }

    // Expect an end token for a for loop ('0' or 'n')
    fn expect_end_token(&mut self) -> Fexpr {
        match self.peek().cloned() {
            Some(Token::Number(0)) => {
                self.current += 1;
                Fexpr::Number(0)
            }
            Some(Token::Identifier(name)) if name == "n" => {
                self.current += 1;
                Fexpr::Var(name.clone())
            }
            _ => panic!("Expected end token to be '0' or 'n'"),
        }
    }
}


//print ast
pub fn print_ast(stmts: &Vec<Stmt>, indent: usize) {
    for stmt in stmts {
        print_stmt(stmt, indent);
    }
}

fn print_stmt(stmt: &Stmt, indent: usize) {
    let indent_str = " ".repeat(indent);

    match stmt {
        Stmt::Print(expr) => {
            println!("{}Print:", indent_str);
            print_pexpr(expr, indent + 2);
        }
        Stmt::For0(var, body) => {
            println!("{}For0 {}:", indent_str, var);
            print_ast(body, indent + 2);
        }
        Stmt::For1(var, body) => {
            println!("{}For1 {}:", indent_str, var);
            print_ast(body, indent + 2);
        }
        Stmt::If(condition, then_branch) => {
            println!("{}If:", indent_str);
            print_bexpr(condition, indent + 2);
            println!("{}Then:", indent_str);
            print_ast(then_branch, indent + 2);
        }
    }
}

fn print_pexpr(expr: &Pexpr, indent: usize) {
    let indent_str = " ".repeat(indent);
    match expr {
        Pexpr::Label(label) => println!("{}Label: {}", indent_str, label),
        Pexpr::Str(s) => println!("{}Str: {}", indent_str, s),
    }
}

fn print_bexpr(expr: &Bexpr, indent: usize) {
    let indent_str = " ".repeat(indent);
    match expr {
        Bexpr::Var(var) => println!("{}Var: {}", indent_str, var),
        Bexpr::Str(s) => println!("{}Str: {}", indent_str, s),
        Bexpr::LessEqual(left, right) => {
            println!("{}LessEqual:", indent_str);
            print_bexpr(left, indent + 2);
            print_bexpr(right, indent + 2);
        }
        Bexpr::Less(left, right) => {
            println!("{}Less:", indent_str);
            print_bexpr(left, indent + 2);
            print_bexpr(right, indent + 2);
        }
        Bexpr::Equal(left, right) => {
            println!("{}Equal:", indent_str);
            print_bexpr(left, indent + 2);
            print_bexpr(right, indent + 2);
        }
        Bexpr::NotEqual(left, right) => {
            println!("{}NotEqual:", indent_str);
            print_bexpr(left, indent + 2);
            print_bexpr(right, indent + 2);
        }
        Bexpr::GreaterEqual(left, right) => {
            println!("{}GreaterEqual:", indent_str);
            print_bexpr(left, indent + 2);
            print_bexpr(right, indent + 2);
        }
        Bexpr::Greater(left, right) => {
            println!("{}Greater:", indent_str);
            print_bexpr(left, indent + 2);
            print_bexpr(right, indent + 2);
        }
        Bexpr::Not(expr) => {
            println!("{}Not:", indent_str);
            print_bexpr(expr, indent + 2);
        }
        Bexpr::Label(label) => println!("{}Label: {}", indent_str, label),
    }
}
