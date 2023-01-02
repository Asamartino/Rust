// Instructions
// Given a string containing brackets [], braces {}, parentheses (), or any combination thereof,
// verify that any and all pairs are matched and nested correctly.

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut vec = Vec::new();
    for c in string.chars() {
        if c == '{' || c == '(' || c == '[' || c == '}' || c == ')' || c == ']' {
            vec.push(c);
        }
        if c == '}' && vec.get(vec.len().wrapping_sub(2)) == Some(&'{')
            || c == ')' && vec.get(vec.len().wrapping_sub(2)) == Some(&'(')
            || c == ']' && vec.get(vec.len().wrapping_sub(2)) == Some(&'[')
        {
            vec.pop();
            vec.pop();
        }
    }
    return vec.is_empty();
}
