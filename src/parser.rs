use crate::lexer::Token;
use crate::value::JsonValue;
use std::collections::HashMap;
use std::iter::Peekable;
use std::vec::IntoIter;

pub fn parse(tokens: Vec<Token>) -> Result<JsonValue, String> {
    let mut iter = tokens.into_iter().peekable();
    let value = parse_value(&mut iter)?;

    if iter.peek().is_some() {
        return Err("Unexpected extra tokens after valid JSON".to_string());
    }

    Ok(value)
}

fn parse_value(iter: &mut Peekable<IntoIter<Token>>) -> Result<JsonValue, String> {
    match iter.next(){
        Some(Token::LeftBrace) => parse_object(iter),
        Some(Token::LeftBracket) => parse_array(iter),
        Some(Token::String(s)) => Ok(JsonValue::String(s)),
        Some(Token::Number(n)) => Ok(JsonValue::Number(n)),
        Some(Token::Null) => Ok(JsonValue::Null),
        Some(Token::True) => Ok(JsonValue::Boolean(true)),
        Some(Token::False) => Ok(JsonValue::Boolean(false)),
        Some(other) => Err(format!("Unknown token: {:?}", other)),
        None => Err("Unexpected end of input".to_string()),
    }
}

//moved old code into new function to facilitate recursion
fn parse_object(iter: &mut Peekable<IntoIter<Token>>) -> Result<JsonValue, String> {
    let mut json_map = HashMap::new();

    // Empty object
    if let Some(Token::RightBrace) = iter.peek() {
        iter.next();
        return Ok(JsonValue::Object(json_map));
    }

    //allow it to handle multiple keys per Json, which means loops

    loop{
        //first part of proper json is key. after googling, keys must be strings :thumbsup:
        let key = match iter.next() {
            Some(Token::String(some_string)) => some_string,
            _ => return Err("Keys must be Strings".to_string()),
        };
        //colon
        match iter.next() {
            Some(Token::Colon) => {}
            _ => return Err("json structure should be key:value".to_string()),
        }
        // recursion :sob:   Also '?' after some function allows errors to bubble up automatically
        let value = parse_value(iter)?;        
        //ok, now that there's a whole key:value, have to put it in the map, but use the recursion from previous line instead of String(value) like before
        json_map.insert(key,value);

        //from hence, only rightbrace and comma are allowed
        match iter.next() {
            Some(Token::RightBrace) => break,
            Some(Token::Comma) => {
                //remove string requirement and allow it to just continue the recursion
                continue;
            },
            Some(other) => return Err(format!("Unexpected token {:?}",other)),
            None => return Err("Unexpected end of input".to_string()),
        }
    }

    Ok(JsonValue::Object(json_map))
}

fn parse_array(iter: &mut Peekable<IntoIter<Token>>) -> Result<JsonValue, String> {
    let mut json_array = Vec::new();

    if let Some(Token::RightBracket) = iter.peek(){
        iter.next();
        return Ok(JsonValue::Array(json_array));
    }

    loop {
        let item = parse_value(iter)?;
        json_array.push(item);

        match iter.next() {
            Some(Token::RightBracket) => break,
            Some(Token::Comma) => continue,
            Some(other) => return Err(format!("Unexpected token found: {:?}",other)),
            None => return Err("Unexpected end of input.".to_string()),
        }
    }

    Ok(JsonValue::Array(json_array))
}