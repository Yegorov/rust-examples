use std::time;

mod http;

fn main() {
    println!("Hello, world!");
    println!("{}", welcome("Artem"));
    println!("{}", welcome(&String::from("Rust")));
    http::get_ip();
    let t = time::SystemTime::now();
    println!("body: {}", http::get_body());
    println!("duration ms: {}", t.elapsed().unwrap().as_millis());
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
