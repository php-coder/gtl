#[cfg(not(test))]
fn main() {
    println!("{:}",
             text_to_tokens("create function main")
                 .iter()
                 .map(|ref tok| (*tok).to_string())
                 .collect::<Vec<_>>()
                 .join(""));
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

}
