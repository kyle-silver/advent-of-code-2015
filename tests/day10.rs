fn look_and_say(n: &str) -> String {
    let mut ans = Vec::new();
    let mut chars = n.chars().peekable();
    while let Some(c) = chars.next() {
        let mut repetitions = 1;
        while let Some(&next) = chars.peek() {
            if next != c {
                break;
            }
            repetitions += 1;
            chars.next();
        }
        ans.push(format!("{}", repetitions));
        ans.push(c.into());
    }
    ans.into_iter().collect()
}

#[test]
fn part1() {
    let mut val: String = "3113322113".into();
    for _ in 0..40 {
        val = look_and_say(&val);
    }
    println!("Day 10, part 1: {}", val.len());
}

#[test]
fn part2() {
    let mut val: String = "3113322113".into();
    for _ in 0..50 {
        val = look_and_say(&val);
    }
    println!("Day 10, part 2: {}", val.len());
}
