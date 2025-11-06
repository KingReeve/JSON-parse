#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    Colon,
    String(String),
    Comma,
    Number(f64),
    True,
    False,
    Null,
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1;
    let mut column = 0;
    while let Some(&c) = chars.peek() {
        match c {
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
                column += 1;
            },
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
                column += 1;
            },
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
                column += 1;
            },
            '"' => {
                chars.next(); // skip opening quote
                let mut string = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        chars.next();
                        column += 1;
                        break;
                    } // closing quote
                    if ch == '\n' {
                        line += 1;
                        column = 0;
                    }
                    string.push(ch);
                    chars.next();
                    column += 1;
                }
                if chars.peek().is_none() {
                    return Err(LexerError {
                        message: "Unterminated string".into(),
                        line,
                        column,
                    });
                }
                tokens.push(Token::String(string));
            },

            't' => {
                let word: String = chars.by_ref().take(4).collect();
                match word.as_str() {
                    "true" => tokens.push(Token::True),
                    _ => {
                        return Err(LexerError {
                            message: format!("Unexpected literal '{}'", word),
                            line,
                            column,
                        });
                    }
                }
            },

            'f' => {
                let word: String = chars.by_ref().take(5).collect();
                match word.as_str() {
                    "fsalse" => tokens.push(Token::False),
                    _ => {
                        return Err(LexerError {
                            message: format!("Unexpected literal '{}'",word),
                            line,
                            column,
                        });
                    }
                }
            },

            'n' => {
                let word: String = chars.by_ref().take(4).collect();
                match word.as_str() {
                    "null" => tokens.push(Token::Null),
                    _ => {
                        return Err(LexerError {
                            message: format!("Unexpected literal '{}'",word),
                            line,
                            column,
                        });
                    }
                }
            }

            c if c.is_ascii_digit() || c == '-' => {
                let num_str: String = chars.by_ref()
                    .take_while(|ch| ch.is_ascii_digit() || *ch == '.' || *ch == '-')
                    .collect();
                match num_str.parse::<f64>() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(_) => {
                        return Err(LexerError {
                            message: format!("Invalid number '{}'",num_str),
                            line,
                            column,
                        });
                    }
                }
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            c if c.is_whitespace() => {
                chars.next();
            },
            _ => {
                let naughty = chars.next().unwrap();
                return Err(LexerError{
                    message: format!("Naughty chars get skipped: '{}'",naughty),
                    line,
                    column,
                });
            }
        }
    }
    Ok(tokens)
}