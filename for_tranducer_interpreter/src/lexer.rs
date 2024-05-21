// Define the possible tokens in the language
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    For,                  // 'for' keyword
    In,                   // 'in' keyword
    Print,                // 'print' keyword
    Identifier(String),   // Variable names
    Number(i32),          // Numeric literals
    LeftBrace,            // '{' character
    RightBrace,           // '}' character
    LeftParen,            // '(' character
    RightParen,           // ')' character
    DotDot,               // '..' range operator
}

// Function to tokenize the input string
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();                 // Vector to hold the tokens
    let mut chars = input.chars().peekable();    // Peekable iterator over the input characters

    // Loop through the characters
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' | '\t' => {
                // Skip whitespace characters
                chars.next();
            }
            'f' if input[chars.clone().count()..].starts_with("for") => {
                // Match the 'for' keyword
                tokens.push(Token::For);
                chars.nth(2); // Consume 'for'
            }
            'i' if input[chars.clone().count()..].starts_with("in") => {
                // Match the 'in' keyword
                tokens.push(Token::In);
                chars.nth(1); // Consume 'in'
            }
            'p' if input[chars.clone().count()..].starts_with("print") => {
                // Match the 'print' keyword
                tokens.push(Token::Print);
                chars.nth(4); // Consume 'print'
            }
            '{' => {
                // Match the '{' character
                tokens.push(Token::LeftBrace);
                chars.next();
            }
            '}' => {
                // Match the '}' character
                tokens.push(Token::RightBrace);
                chars.next();
            }
            '(' => {
                // Match the '(' character
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                // Match the ')' character
                tokens.push(Token::RightParen);
                chars.next();
            }
            '.' if input[chars.clone().count()..].starts_with("..") => {
                // Match the '..' range operator
                tokens.push(Token::DotDot);
                chars.nth(1); // Consume '..'
            }
            ch if ch.is_numeric() => {
                // Match numeric literals
                let mut num = 0;
                while let Some(&ch) = chars.peek() {
                    if let Some(digit) = ch.to_digit(10) {
                        num = num * 10 + digit as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num));
            }
            ch if ch.is_alphabetic() => {
                // Match variable names (identifiers)
                let mut identifier = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() {
                        identifier.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(identifier));
            }
            _ => panic!("Unexpected character: {}", ch), // Handle unexpected characters
        }
    }

    tokens // Return the vector of tokens
}
