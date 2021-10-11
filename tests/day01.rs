const INPUT: &str = include_str!("res/01.txt");

#[test]
fn part1() {
    let ans: i32 = INPUT
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();
    println!("Day 1, part 1: {}", ans);
}

#[test]
fn part2() {
    let instructions: Vec<i32> = INPUT
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .collect();
    let mut pos = 0;
    let mut sum = 0;
    for (i, instr) in instructions.iter().enumerate() {
        sum += instr;
        if sum < 0 {
            pos = i + 1;
            break;
        }
    }
    println!("Day 1, part 2: {}", pos);
}
