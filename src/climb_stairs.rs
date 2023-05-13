pub fn climb_stairs(n: i32) -> i32 {
    match n {
        1 => return 1,
        2 => return 2,
        _ => {}
    }
    let mut ways: [i32; 2] = [1, 2];
    let mut step: i32 = 3;
    while step < n {
        let new_step: i32 = ways[0] + ways[1];
        let previous_step: i32 = ways[1];
        ways = [previous_step, new_step];
        step += 1;
    }

    ways.iter().sum()
}

pub fn run() {
    let input = 5;
    println!("{}", climb_stairs(input));
}
