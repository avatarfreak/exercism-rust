pub fn hello(string: Option<&str>) -> String {
    match string {
        Some(x) => format!("Hello, {}!", x),
        None => format!("Hello, World!"),
    }
}
