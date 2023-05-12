pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits_clone = digits.clone();
    let mut cnt = 0;
    let mut no_more_increases: bool = false;
    let len = digits_clone.len() - 1;

    while !no_more_increases {
        digits_clone[len - cnt] += 1;

        if digits_clone[len - cnt] == 10 {
            digits_clone[len - cnt] = 0;
            cnt += 1;
            if cnt >= len + 1 {
                digits_clone.push(0);
                digits_clone[0] = 1;
                break;
            }
        } else {
            no_more_increases = true;
        }
    }

    digits_clone
}

pub fn plus_one_easier(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    for v in digits.iter_mut().rev() {
        let sum = *v + 1;
        *v = sum % 10;
        if sum < 10 {
            return digits;
        }
    }
    [&vec![1], &digits[..]].concat()
}

pub fn run() {
    let input = [9, 9].to_vec();
    println!("{:?}", plus_one_easier(input));
}
