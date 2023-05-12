pub fn length_of_last_word(s: String) -> i32 {
    let words: Vec<&str> = s.split(" ").filter(|&x| !x.is_empty()).collect();
    println!("{:?}", words);
    let res = words[words.len() - 1].len();

    res as i32
}

pub fn run() {
    let input = String::from("luffy is still joyboy");
    println!("{}", length_of_last_word(input));
}
