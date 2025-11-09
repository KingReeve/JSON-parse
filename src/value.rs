#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(std::collections::HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}