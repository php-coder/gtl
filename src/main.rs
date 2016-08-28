#[cfg(not(test))]
fn main() {
    println!("Here will be a garden!");
    tokens_from_string();
}

fn tokens_from_string() {
}

#[cfg(test)]
mod tests {
    use super::tokens_from_string;

    #[test]
    fn should_handle_empty_string() {
        tokens_from_string();
    }

}
