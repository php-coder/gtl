#[cfg(not(test))]
fn main() {
    println!("{}",
             tokens_to_string(text_to_tokens("create function main")));
}

fn text_to_tokens(text: &'static str) -> Vec<Box<Token>> {
    let parts: Vec<&str> = text.split(" ").collect();
    let mut tokens: Vec<Box<Token>> = Vec::new();

    if parts.len() == 3 && parts[0] == "create" && parts[1] == "function" {
        tokens.push(Box::new(SimpleToken::new("fn")));
        tokens.push(Box::new(SimpleToken::new(" ")));
        tokens.push(Box::new(SimpleToken::new(parts[2])));
        tokens.push(Box::new(SimpleToken::new("(")));
        tokens.push(Box::new(SimpleToken::new(")")));
        tokens.push(Box::new(SimpleToken::new(" ")));
        tokens.push(Box::new(SimpleToken::new("{")));
        tokens.push(Box::new(SimpleToken::new("\n")));
        tokens.push(Box::new(SimpleToken::new("}")));
    }

    tokens
}

fn tokens_to_string(tokens: Vec<Box<Token>>) -> String {
    tokens.iter()
        .map(|ref tok: &Box<Token>| (*tok).to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn partition_string(string: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if string.is_empty() {
        return result;
    }

    let mut start: usize = 0;
    let mut after_separator: bool = false;

    // this version is slow because it uses chars and not bytes, but it's correct
    for (pos, c) in string.char_indices() {
        if c.is_whitespace() || c == '{' || c == '}' || c == '(' || c == ')' {
            if pos > 0 {
                result.push(string[start..pos].to_string());
                start = pos;
            }
            after_separator = true;
        } else if after_separator {
            result.push(string[start..pos].to_string());
            start = pos;
            after_separator = false;
        }
    }
    result.push(string[start..string.len()].to_string());
    result
}

fn code_to_tokens(code: String) -> Vec<SimpleToken> {
    partition_string(code)
        .into_iter()
        .map(|part: String| SimpleToken::from_string(part))
        .collect()
}

trait Token {
    fn to_string(&self) -> String;
}

#[derive(Debug,PartialEq)]
struct SimpleToken {
    name: String,
}

impl SimpleToken {
    fn new(name: &'static str) -> SimpleToken {
        SimpleToken { name: name.to_string() }
    }
    fn from_string(name: String) -> SimpleToken {
        SimpleToken { name: name }
    }
}

impl Token for SimpleToken {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::text_to_tokens;

    #[test]
    fn text_to_tokens_returns_empty_tokens_for_empty_string() {
        assert!(text_to_tokens("").len() == 0);
    }

    #[test]
    fn text_to_tokens_returns_function_tokens() {
        let tokens: Vec<String> = text_to_tokens("create function foo")
            .iter()
            .map(|ref tok| (*tok).to_string())
            .collect::<Vec<_>>();
        assert_eq!(vec!["fn", " ", "foo", "(", ")", " ", "{", "\n", "}"],
                   tokens);
    }

    use super::Token;
    use super::SimpleToken;
    use super::tokens_to_string;

    #[test]
    fn tokens_to_string_returns_empty_string_for_empty_tokens() {
        let tokens: Vec<Box<Token>> = Vec::new();
        assert_eq!(tokens_to_string(tokens), "");
    }

    #[test]
    fn tokens_to_string_returns_tokens_representation() {
        let tokens: Vec<Box<Token>> = vec![Box::new(SimpleToken::new("fn")),
                                           Box::new(SimpleToken::new(" ")),
                                           Box::new(SimpleToken::new("main")),
                                           Box::new(SimpleToken::new("(")),
                                           Box::new(SimpleToken::new(")")),
                                           Box::new(SimpleToken::new(" ")),
                                           Box::new(SimpleToken::new("{")),
                                           Box::new(SimpleToken::new("\n")),
                                           Box::new(SimpleToken::new("}"))];
        assert_eq!(tokens_to_string(tokens), "fn main() {\n}");
    }

    use super::partition_string;

    #[test]
    fn partition_string_returns_empty_vector_for_empty_string() {
        assert!(partition_string("".to_string()).len() == 0);
    }

    #[test]
    fn partition_string_returns_partitioned_string() {
        assert_eq!(partition_string("(test){ }".to_string()),
                   vec!["(".to_string(),
                        "test".to_string(),
                        ")".to_string(),
                        "{".to_string(),
                        " ".to_string(),
                        "}".to_string()]);
    }

    #[test]
    fn partition_string_should_handle_string_with_separators_only() {
        assert_eq!(partition_string("   ".to_string()),
                   vec![" ".to_string(), " ".to_string(), " ".to_string()]);
    }

    #[test]
    fn partition_string_should_handle_utf8_string() {
        assert_eq!(partition_string("忠犬ハチ公 (忠犬ハチ公)".to_string()),
                   vec!["忠犬ハチ公".to_string(),
                        " ".to_string(),
                        "(".to_string(),
                        "忠犬ハチ公".to_string(),
                        ")".to_string()]);
    }

    use super::code_to_tokens;

    #[test]
    fn code_to_tokens_returns_empty_tokens_for_empty_code() {
        assert!(code_to_tokens("".to_string()).len() == 0);
    }

    #[test]
    fn code_to_tokens_returns_function_tokens() {
        let expected: Vec<SimpleToken> = vec![SimpleToken::new("fn"),
                                              SimpleToken::new(" "),
                                              SimpleToken::new("test"),
                                              SimpleToken::new("("),
                                              SimpleToken::new(")"),
                                              SimpleToken::new(" "),
                                              SimpleToken::new("{"),
                                              SimpleToken::new("\n"),
                                              SimpleToken::new("}")];
        let tokens: Vec<SimpleToken> = code_to_tokens("fn test() {\n}".to_string());

        assert_eq!(expected, tokens);
    }

}
