// V1 with for loop
pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = vec![];
    if len > digits.len() {
        return result;
    }
    if len == 0 { // mandatory to pass test, but why? -> mistake in test ?
        result.push(String::from(""));
    }
    for i in 0..digits.len() {
        if i + len <= digits.len() {
            let s = String::from(&digits[i..i + len]);
            result.push(s);
        }
    }
    result
}
