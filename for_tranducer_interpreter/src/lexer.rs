#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    For,
    In,
    Print,
    Identifier(String),
    Number(i32),
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    DotDot,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' | '\t' => {
                chars.next();
            }
            'f' if input[chars.clone().count()..].starts_with("for") => {
                tokens.push(Token::For);
                chars.nth(2);
            }
            'i' if input[chars.clone().count()..].starts_with("in") => {
                tokens.push(Token::In);
                chars.nth(1);
            }
            'p' if input[chars.clone().count()..].starts_with("print") => {
                tokens.push(Token::Print);
                chars.nth(4);
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            '.' if input[chars.clone().count()..].starts_with("..") => {
                tokens.push(Token::DotDot);
                chars.nth(1);
            }
            ch if ch.is_numeric() => {
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
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    tokens
}
