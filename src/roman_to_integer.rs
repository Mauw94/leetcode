use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut roman_hashmap: HashMap<char, i32> = HashMap::new();
    roman_hashmap.insert('I', 1);
    roman_hashmap.insert('V', 5);
    roman_hashmap.insert('X', 10);
    roman_hashmap.insert('L', 50);
    roman_hashmap.insert('C', 100);
    roman_hashmap.insert('D', 500);
    roman_hashmap.insert('M', 1000);

    let mut result: i32 = 0;
    let mut temp: i32 = 0;

    for c in s.chars().rev() {
        let val = roman_hashmap.get(&c).unwrap();

        if val >= &temp {
            result += val;
        } else {
            result -= val;
        }

        temp = *val;
    }

    result
}

pub fn run() {
    let input = String::from("MCMXCIV");
    println!("{}", roman_to_int(input));
}
