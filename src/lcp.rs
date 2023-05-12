use std::cmp;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix: &str = &strs[0].to_string().clone();

    for i in 1..strs.len() {
        let s = &strs[i];
        let min = cmp::min(prefix.len(), strs[i].len());
        prefix = prefix.get(0..min).unwrap();
        for k in 0..s.len() {
            if s.chars().nth(k) != prefix.chars().nth(k) {
                prefix = prefix.get(0..k).unwrap();
                break;
            }
        }
    }

    prefix.to_string()
}

pub fn run() {
    let strings: Vec<String> = ["ab", "a"].map(String::from).to_vec();
    let res = longest_common_prefix(strings);
    println!("{}", &res);
}
