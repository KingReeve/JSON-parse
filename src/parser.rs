use crate::lexer::Token;
use crate::value::JsonValue;
use std::collections::HashMap;

pub fn parse(tokens: Vec<Token>) -> Result<JsonValue, String> {
    let mut iter = tokens.into_iter().peekable();
    let mut json_map = HashMap::new();


    match iter.next() {
        Some(Token::LeftBrace) => {}
        _ => return Err("Json objects should start with '{'".to_string()),
    };
    // Empty object
    if let Some(Token::RightBrace) = iter.peek() {
        iter.next();
        return Ok(JsonValue::Object(json_map));
    }

    //allow it to handle multiple keys per Json, which means loops

    loop{
        //first part of proper json is key
        let key = match iter.next() {
            Some(Token::String(some_string)) => some_string,
            _ => return Err("There should be a string inside this key for now".to_string()),
        };

        match iter.next() {
            Some(Token::Colon) => {}
            _ => return Err("json structure should be key:value".to_string()),
        }

        let value = match iter.next() {
            Some(Token::String(some_string)) => some_string,
            _ => return Err("There should be a string inside this value for now".to_string()),
        };
        
        //ok, now that there's a whole key:value, have to put it in the map
        json_map.insert(key,JsonValue::String(value));

        //from hence, only rightbrace and comma are allowed
        match iter.next() {
            Some(Token::RightBrace) => break,
            Some(Token::Comma) => {
                if !matches!(iter.peek(), Some(Token::String(_))) {
                    return Err("Expected string key after comma".to_string());
                }
            },
            Some(other) => return Err(format!("Unexpected character {:?}",other)),
            None => return Err("Unexpected end of input".to_string()),
        }
    }

    Ok(JsonValue::Object(json_map))
}