#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    Colon,
    String(String),
    Comma,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
            },
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();   
            },
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            },
            '"' => {
                chars.next(); // skip opening quote
                let mut string = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        chars.next(); break;
                    } // closing quote
                    string.push(ch);
                    chars.next();
                }
                tokens.push(Token::String(string));
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            c if c.is_whitespace() => {
                chars.next();
            },
            _ => {
                chars.next(); //naughty chars get skipped
            }
        }
    }

    tokens
}