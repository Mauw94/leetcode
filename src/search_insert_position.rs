pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut res = 0;
    for (idx, n) in nums.iter().enumerate() {
        if n.eq(&target) {
            res = idx as i32;
        } else {
            if target > nums[idx] && (idx + 1) >= nums.len() {
                res = (idx + 1) as i32
            } else if target > nums[idx] && target < nums[idx + 1] {
                res = (idx + 1) as i32
            }
        }
    }

    res
}

pub fn run() {
    let input = [1, 3, 5, 6].to_vec();
    let target = 5;
    println!("{}", search_insert(input, target));
}
