#[cfg(not(test))]
fn main() {
    println!("{:}", text_to_tokens("create function main").join(""));
}

fn text_to_tokens(text: &str) -> Vec<String> {
    let parts: Vec<&str> = text.split(" ").collect();
    let mut tokens: Vec<String> = Vec::new();
    if parts.len() == 0 {
        return tokens;
    }
    if parts.len() == 3 && parts[0] == "create" && parts[1] == "function" {
        tokens.push("fn".to_string());
        tokens.push(" ".to_string());
        tokens.push(parts[2].to_string());
        tokens.push("(".to_string());
        tokens.push(")".to_string());
        tokens.push(" ".to_string());
        tokens.push("{".to_string());
        tokens.push("\n".to_string());
        tokens.push("}".to_string());
    }
    tokens
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
        let tokens: Vec<String> = text_to_tokens("create function foo");
        assert_eq!(vec!["fn", " ", "foo", "(", ")", " ", "{", "\n", "}"],
                   tokens);
    }

}
