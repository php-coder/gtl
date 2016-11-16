#[cfg(not(test))]
use std::env;

#[cfg(not(test))]
use std::fs::{OpenOptions, File};

#[cfg(not(test))]
// Write is required in order to use write_all()
use std::io::{BufWriter, Write};

#[cfg(not(test))]
use std::io::Error;

#[cfg(not(test))]
fn main() {
    let ref tokens: Vec<Token> = text_to_tokens("create function main");
    let mut code: String = tokens_to_string(tokens);
    code.push_str("\n");

    // FIXME: from std::env::args() doc: "The returned iterator will panic
    // during iteration if any argument to the process is not valid unicode"
    let argv: Vec<String> = env::args().collect();
    if argv.len() == 3 && argv[1] == "-f" && argv[2] != "-" {
        let file: &str = &argv[2];
        let code: i32  = match write_str_to_file(file, &code) {
            Ok(_)  => 0,
            Err(e) => {
                // TODO: write to stderr
                println!("ERROR: couldn't write to file '{}': {}", file, e);
                1
            }
        };
        std::process::exit(code);
    } else {
        print!("{}", code);
    }

}

#[cfg(not(test))]
fn write_str_to_file(filename: &str, text: &String) -> Result<(), Error> {
    let file: File = try!(OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename));

    let mut buffer: BufWriter<&File> = BufWriter::new(&file);
    try!(buffer.write_all(text.as_bytes()));

    Ok(())
}

fn text_to_tokens(text: &str) -> Vec<Token> {
    let parts: Vec<&str> = text.split(" ").collect();
    let mut tokens: Vec<Token> = Vec::new();

    if parts.len() == 3 && parts[0] == "create" && parts[1] == "function" {
        tokens.push(Token::new("fn"));
        tokens.push(Token::new(" "));
        tokens.push(Token::new(parts[2]));
        tokens.push(Token::new("("));
        tokens.push(Token::new(")"));
        tokens.push(Token::new(" "));
        tokens.push(Token::new("{"));
        tokens.push(Token::new("\n"));
        tokens.push(Token::new("}"));
    }

    tokens
}

fn tokens_to_string(tokens: &[Token]) -> String {
    tokens.iter()
        .map(|tok: &Token| tok.to_string())
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
fn partition_string<'a>(string: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    if string.is_empty() {
        return result;
    }

    let mut start: usize = 0;
    let mut after_separator: bool = false;

    // this version is slow because it uses chars and not bytes, but it's correct
    for (pos, c) in string.char_indices() {
        if c.is_whitespace() || c == '{' || c == '}' || c == '(' || c == ')' {
            if pos > 0 {
                result.push(&string[start..pos]);
                start = pos;
            }
            after_separator = true;
        } else if after_separator {
            result.push(&string[start..pos]);
            start = pos;
            after_separator = false;
        }
    }
    result.push(&string[start..]);
    result
}

#[cfg(test)]
fn code_to_tokens(code: &str) -> Vec<Token> {
    partition_string(code)
        .into_iter()
        .map(|part: &str| Token::new(part))
        .collect()
}

#[derive(Debug,PartialEq)]
struct Token {
    name: String,
}

impl Token {
    fn new<S>(name: S) -> Token
        where S: Into<String>
    {
        Token { name: name.into() }
    }
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::text_to_tokens;

    #[test]
    fn text_to_tokens_returns_empty_tokens_for_empty_string() {
        assert_eq!(text_to_tokens("").len(), 0);
    }

    #[test]
    fn text_to_tokens_returns_function_tokens() {
        let tokens: Vec<String> = text_to_tokens("create function foo")
            .iter()
            .map(|tok: &Token| tok.to_string())
            .collect();
        assert_eq!(vec!["fn", " ", "foo", "(", ")", " ", "{", "\n", "}"],
                   tokens);
    }

    use super::Token;
    use super::tokens_to_string;

    #[test]
    fn tokens_to_string_returns_empty_string_for_empty_tokens() {
        let ref tokens: Vec<Token> = Vec::new();
        assert_eq!(tokens_to_string(tokens), "");
    }

    #[test]
    fn tokens_to_string_returns_tokens_representation() {
        let ref tokens: Vec<Token> = vec![Token::new("fn"),
                                      Token::new(" "),
                                      Token::new("main"),
                                      Token::new("("),
                                      Token::new(")"),
                                      Token::new(" "),
                                      Token::new("{"),
                                      Token::new("\n"),
                                      Token::new("}")];
        assert_eq!(tokens_to_string(tokens), "fn main() {\n}");
    }

    use super::partition_string;

    #[test]
    fn partition_string_returns_empty_vector_for_empty_string() {
        assert_eq!(partition_string("").len(), 0);
    }

    #[test]
    fn partition_string_returns_partitioned_string() {
        assert_eq!(partition_string("(test){ }"),
                   vec!["(", "test", ")", "{", " ", "}"]);
    }

    #[test]
    fn partition_string_should_handle_string_with_separators_only() {
        assert_eq!(partition_string("   "),
                   vec![" ", " ", " "]);
    }

    #[test]
    fn partition_string_should_handle_utf8_string() {
        assert_eq!(partition_string(&"忠犬ハチ公 (忠犬ハチ公)".to_string()),
                   vec!["忠犬ハチ公", " ", "(", "忠犬ハチ公", ")"]);
    }

    use super::code_to_tokens;

    #[test]
    fn code_to_tokens_returns_empty_tokens_for_empty_code() {
        assert_eq!(code_to_tokens("").len(), 0);
    }

    #[test]
    fn code_to_tokens_returns_function_tokens() {
        let expected: Vec<Token> = vec![Token::new("fn"),
                                        Token::new(" "),
                                        Token::new("test"),
                                        Token::new("("),
                                        Token::new(")"),
                                        Token::new(" "),
                                        Token::new("{"),
                                        Token::new("\n"),
                                        Token::new("}")];
        let tokens: Vec<Token> = code_to_tokens("fn test() {\n}");

        assert_eq!(expected, tokens);
    }

}
