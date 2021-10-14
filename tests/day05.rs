const INPUT: &str = include_str!("res/05.txt");

fn nice(token: &str) -> bool {
    let mut seen_vowels = [0, 0, 0, 0, 0];
    let mut has_double = false;
    let mut chars = token.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            'a' => seen_vowels[0] += 1,
            'e' => seen_vowels[1] += 1,
            'i' => seen_vowels[2] += 1,
            'o' => seen_vowels[3] += 1,
            'u' => seen_vowels[4] += 1,
            _ => {}
        };
        if let Some(&next) = chars.peek() {
            if c == next {
                has_double = true;
            }
            match c {
                'a' => {
                    if next == 'b' {
                        return false;
                    }
                }
                'c' => {
                    if next == 'd' {
                        return false;
                    }
                }
                'p' => {
                    if next == 'q' {
                        return false;
                    }
                }
                'x' => {
                    if next == 'y' {
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    let vowel_count: i32 = seen_vowels.iter().sum();
    return vowel_count >= 3 && has_double;
}

#[test]
fn part1() {
    let ans = INPUT.lines().map(nice).filter(|b| *b).count();
    println!("Day 5, part 1: {}", ans);
    println!("{}", nice("aabbccdd"))
}

fn double_double(token: &[char]) -> bool {
    for i in 0..token.len() - 1 {
        // let base = (token[i], token[i + 1]);
        let base = &token[i..i + 2];
        for window in token[i + 2..].windows(2) {
            if base == window {
                return true;
            }
        }
    }
    return false;
}

fn repeat_with_split(token: &[char]) -> bool {
    token.windows(3).any(|w| w[0] == w[2])
}

fn nicer(token: &str) -> bool {
    let chars: Vec<_> = token.chars().collect();
    double_double(&chars) && repeat_with_split(&chars)
}

#[test]
fn part2() {
    // let chars: Vec<char> = "aabcdefgaa".chars().collect();
    // println!("{}", double_double(&chars));
    let ans = INPUT.lines().map(nicer).filter(|b| *b).count();
    println!("Day 5, part 2: {}", ans);
}
