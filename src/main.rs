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

trait Token {
    fn to_string(&self) -> &'static str;
}

struct SimpleToken {
    name: &'static str,
}

impl SimpleToken {
    fn new(name: &'static str) -> SimpleToken {
        SimpleToken { name: name }
    }
}

impl Token for SimpleToken {
    fn to_string(&self) -> &'static str {
        self.name
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
        let tokens: Vec<&'static str> = text_to_tokens("create function foo")
            .iter()
            .map(|ref tok| (*tok).to_string())
            .collect();
        assert_eq!(vec!["fn", " ", "foo", "(", ")", " ", "{", "\n", "}"],
                   tokens);
    }

    use super::Token;
    use super::SimpleToken;
    use super::tokens_to_string;

    #[test]
    fn tokens_to_string_returns_empty_string_for_empty_tokens() {
        let tokens: Vec<Box<Token>> = Vec::new();
        assert!(tokens_to_string(tokens) == "");
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

}
