pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![vec![1]];

    for i in 1..num_rows {
        let mut row = vec![1];
        for j in 1..i as usize {
            row.push(ans[i as usize - 1][j - 1] + ans[i as usize - 1][j]);
        }
        row.push(1);
        ans.push(row);
    }

    ans
}

pub fn run() {
    let num_rows = 5;
    println!("{:?}", generate(num_rows));
}
