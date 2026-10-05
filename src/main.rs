fn main() {
    println!("Hello, world!");
    println!("{}", welcome("Artem"));
    println!("{}", welcome(&String::from("Rust")))
}

fn welcome(user: &str) -> String {
    format!("Hello {}", user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome() {
        let user: &str = "Artem";
        assert_eq!("Hello Artem", welcome(user));
    }

    #[test]
    fn test_welcome2() {
        let user = String::from("Artem");
        assert_eq!("Hello Artem", welcome(user.as_str()));
    }
}
