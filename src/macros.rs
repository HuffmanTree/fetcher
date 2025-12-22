#[macro_export]
macro_rules! debug {
    () => {};
    ($($arg:tt)*) => {{
        if std::env::var("DEBUG") == Ok(String::from("1")) {
            print!("[DEBUG] ");
            println!($($arg)*);
        }
    }};
}
