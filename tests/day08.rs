const INPUT: &str = include_str!("res/08.txt");

fn escaped_len(token: &str) -> usize {
    let mut count = 0;
    let mut chars = token.chars().peekable();
    while let Some(c) = chars.next() {
        count += match c {
            '"' => 0,
            '\\' => match chars.next().unwrap() {
                '"' => 1,
                '\\' => 1,
                'x' => {
                    chars.next();
                    chars.next();
                    1
                }
                _ => unreachable!("invalid escape sequence"),
            },
            _ => 1,
        }
    }
    return count;
}

fn encoding_differential(token: &str) -> usize {
    token.len() - escaped_len(token)
}

fn reencoded_length(token: &str) -> usize {
    let mut count = 0;
    let mut chars = token.chars();
    while let Some(c) = chars.next() {
        count += match c {
            '"' => 2,
            '\\' => 2,
            _ => 1,
        }
    }
    return count + 2;
}

fn double_encoded_differential(token: &str) -> usize {
    reencoded_length(token) - token.len()
}

#[test]
fn part1() {
    let ans: usize = INPUT.lines().map(encoding_differential).sum();
    println!("Day 8, part 1: {}", ans);
}

#[test]
fn part2() {
    let ans: usize = INPUT.lines().map(double_encoded_differential).sum();
    println!("Day 8, part 2: {}", ans);
}
