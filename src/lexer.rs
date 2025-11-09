use std::iter::Peekable;
use std::str::Chars;

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
    LeftBracket,
    RightBracket,
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
                advance(&mut chars, &mut line, &mut column);
            },
            '}' => {
                tokens.push(Token::RightBrace);
                advance(&mut chars, &mut line, &mut column);
            },
            ':' => {
                tokens.push(Token::Colon);
                advance(&mut chars, &mut line, &mut column);
            },
            '[' => {
                tokens.push(Token::LeftBracket);
                advance(&mut chars, &mut line, &mut column);
            },
            ']' => {
                tokens.push(Token::RightBracket);
                advance(&mut chars, &mut line, &mut column);
            }
            '"' => {
                advance(&mut chars, &mut line, &mut column); // skip opening quote
                let mut string = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        advance(&mut chars, &mut line, &mut column);
                        break;
                    } // closing quote
                    string.push(ch);
                    advance(&mut chars, &mut line, &mut column);
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
                let mut word = String::new();
                for _ in 0..4 {
                    if let Some(ch) = advance(&mut chars, &mut line, &mut column) {
                        word.push(ch);
                    }
                }
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
                let mut word = String::new();
                for _ in 0..5 {
                    if let Some(ch) = advance(&mut chars, &mut line, &mut column) {
                        word.push(ch);
                    }
                }
                match word.as_str() {
                    "false" => tokens.push(Token::False),
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
                let mut word = String::new();
                for _ in 0..4 {
                    if let Some(ch) = advance(&mut chars, &mut line, &mut column) {
                        word.push(ch);
                    }
                }
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
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '-' {
                        num_str.push(ch);
                        advance(&mut chars, &mut line, &mut column);
                    } else {
                        break;
                    }
                }
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
                advance(&mut chars, &mut line, &mut column);
            }
            c if c.is_whitespace() => {
                advance(&mut chars, &mut line, &mut column);
            },
            _ => {
                let naughty = chars.next().unwrap();
                return Err(LexerError{
                    message: format!("Naughty chars get errored: '{}'",naughty),
                    line,
                    column,
                });
            }
        }
    }
    Ok(tokens)
}

fn advance(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) -> Option<char> {
    if let Some(c) = chars.next() {
        if c=='\n'{
            *line+=1;
            *column=0;
        }else{
            *column+=1;
        }
        Some(c)
    }else{
        None
    }
}