pub fn hello() -> String {
    String::from("Hello, world!")
}

#[cfg(test)]
mod test {
    use crate::hello;

    #[test]
    fn say_hello() {
        assert_eq!(hello(), String::from("Hello, world!"));
    }
}
