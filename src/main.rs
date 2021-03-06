fn main() {
    println!("{}", hello_world());
}

fn hello_world() -> String {
    return "Hello World".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(hello_world(), "Hello World".to_string());
    }
}