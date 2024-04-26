pub fn camel_to_snake(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3 / 2);
    let mut text = input.chars();
    if let Some(first) = text.next() {
        result.push(first.to_ascii_lowercase());
        for c in text {
            if c.is_ascii_uppercase() {
                result.push('_');
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }
    }

    result
}
