/// Extract the inner string-like value from an Option
pub fn s<T: ToString>(foo: Option<T>) -> String {
    foo.map_or(String::from(""), |t| t.to_string())
}
