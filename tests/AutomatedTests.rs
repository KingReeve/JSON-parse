use std::fs;
use std::path::Path;
use JSON_parse::lexer::lex;
use JSON_parse::parser::parse;

fn test_json_file(file_path: &str, should_pass: bool) {
    let contents = fs::read_to_string(file_path)
        .expect(&format!("Failed to read test file: {}", file_path));

    let tokens = match lex(&contents) {
        Ok(toks) => toks,
        Err(e) => {
            if should_pass{
                panic!(
                    "Lexing failed for file {}, at line {}, on column {}. error - {}",
                    file_path, e.line, e.column, e.message
                );
            }else{
                return;
            }

        }
    };
    let result = parse(tokens);

    if should_pass {
        assert!(result.is_ok(), "Expected success for file {}, got error {:?}", file_path, result);
    } else {
        assert!(result.is_err(), "Expected error for file {}, but parsing succeeded", file_path);
    }
}

#[test]
fn step1_tests() {
    let base_path = Path::new("tests/step1");
    test_json_file(base_path.join("valid.json").to_str().unwrap(), true);
    test_json_file(base_path.join("invalid.json").to_str().unwrap(), false);
}

#[test]
fn step2_tests() {
    let base_path = Path::new("tests/step2");
    test_json_file(base_path.join("valid.json").to_str().unwrap(), true);
    test_json_file(base_path.join("valid2.json").to_str().unwrap(), true);
    test_json_file(base_path.join("invalid.json").to_str().unwrap(), false);
    test_json_file(base_path.join("invalid2.json").to_str().unwrap(), false);
}