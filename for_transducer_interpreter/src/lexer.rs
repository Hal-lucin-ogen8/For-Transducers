#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    For,
    In,
    Print,
    If,
    Else,
    Identifier(String),
    Number(i32),
    String(String),
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    DotDot,
    LessEqual,
    Less,
    Equal,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' | '\t' => {
                chars.next();
            }
            'f' if chars.clone().take(3).collect::<String>() == "for" => {
                tokens.push(Token::For);
                chars.nth(2);
            }
            'i' if chars.clone().take(2).collect::<String>() == "in" => {
                tokens.push(Token::In);
                chars.nth(1);
            }
            'p' if chars.clone().take(5).collect::<String>() == "print" => {
                tokens.push(Token::Print);
                chars.nth(4);
            }
            'i' if chars.clone().take(2).collect::<String>() == "if" => {
                tokens.push(Token::If);
                chars.nth(1);
            }
            'e' if chars.clone().take(4).collect::<String>() == "else" => {
                tokens.push(Token::Else);
                chars.nth(3);
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
            '.' => {
                chars.next();
                if chars.peek() == Some(&'.') {
                    chars.next();
                    tokens.push(Token::DotDot);
                } else {
                    panic!("Unexpected character: {}", ch);
                }
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less);
                }
            }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::Equal);
                } else {
                    panic!("Unexpected character: {}", ch);
                }
            }
            '"' => {
                chars.next();
                let mut string_literal = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        break;
                    } else {
                        string_literal.push(ch);
                        chars.next();
                    }
                }
                if chars.peek() == Some(&'"') {
                    chars.next();
                    tokens.push(Token::String(string_literal));
                } else {
                    panic!("Unterminated string literal");
                }
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
                match identifier.as_str() {
                    "for" => tokens.push(Token::For),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    // //print all tokens
    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    tokens
}
