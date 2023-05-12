use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index_hs = HashMap::with_capacity(nums.len());

    for (idx, &n) in nums.iter().enumerate() {
        let y = target - n;
        if let Some(&i) = index_hs.get(&y) {
            return [i as i32, idx as i32].to_vec();
        } else {
            index_hs.insert(n, idx);
        }
    }
    vec![]
}

pub fn run() {
    let input = [3, 2, 4].to_vec();
    let target = 6;
    println!("{:?}", two_sum(input, target));
}
