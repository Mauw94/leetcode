pub fn is_valid(s: String) -> bool {
    let mut chars = Vec::new();

    for c in s.chars() {
        match c {
            '{' => chars.push('}'),
            '(' => chars.push(')'),
            '[' => chars.push(']'),
            '}' | ')' | ']' if Some(c) != chars.pop() => return false,
            _ => (),
        }
    }

    chars.is_empty()
}

pub fn run() {
    let input = String::from("()[]{}");
    let res = is_valid(input);
    println!("{}", res);
}
