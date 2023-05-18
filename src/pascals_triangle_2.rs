pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut triangle: Vec<Vec<i32>> = vec![vec![1]];

    for i in 1..row_index + 1 {
        let mut row = vec![1];
        for j in 1..i as usize {
            row.push(triangle[i as usize - 1][j - 1] + triangle[i as usize - 1][j])
        }
        row.push(1);
        triangle.push(row);
    }

    triangle[row_index as usize].clone()
}

pub fn run() {
    let row_index = 3;
    println!("{:?}", get_row(row_index));
}
