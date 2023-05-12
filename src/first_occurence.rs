pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut result: i32 = -1;
    if haystack.contains(needle.as_str()) {
        let idx = haystack.find(needle.as_str()).unwrap();
        result = idx as i32;
    }
    result
}

pub fn run() {
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");

    println!("{}", str_str(haystack, needle));
}
