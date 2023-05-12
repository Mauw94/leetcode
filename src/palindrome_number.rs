pub fn is_palindrome(x: i32) -> bool {
    let mut reversed = String::new();

    for c in x.to_string().chars().rev() {
        reversed.push(c);
    }

    x.to_string().eq(&reversed)
}

pub fn run() {
    let input = 1234321;
    println!("{}", is_palindrome(input));
}
