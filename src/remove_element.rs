pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;

    for n in 0..nums.len() {
        if nums[n] != val {
            nums[k] = nums[n];
            k += 1;
        }
    }

    k as i32
}

pub fn run() {
    let mut input = [3, 2, 2, 3].to_vec();
    let val = 3;
    println!("{}", remove_element(&mut input, val));
}
