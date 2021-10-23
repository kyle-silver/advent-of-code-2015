use std::collections::HashMap;

use itertools::Itertools;

fn notch(password: &mut [char]) {
    let mut carry;
    for i in 0..password.len() - 1 {
        match password[i] {
            'z' => {
                password[i] = 'a';
                carry = true;
            }
            _ => {
                password[i] = (password[i] as u8 + 1) as char;
                carry = false;
            }
        }
        if carry == false {
            return;
        }
    }
}

fn good_letters(password: &[char]) -> bool {
    !password.iter().any(|c| match c {
        'i' | 'o' | 'l' => true,
        _ => false,
    })
}

fn increasing_run(password: &[char]) -> bool {
    for i in 0..password.len() - 2 {
        let vals: Vec<_> = password[i..i + 3].iter().map(|c| *c as u8).collect();
        if vals[1] + 1 == vals[0] && vals[2] + 1 == vals[1] {
            return true;
        }
    }
    return false;
}

fn double_doubles(password: &[char]) -> bool {
    let mut doubles: Vec<Option<char>> = vec![None; password.len()];
    let mut iter = password.iter().enumerate().peekable();
    while let Some((i, &c)) = iter.next() {
        if let Some((_, &next)) = iter.peek() {
            if c == next {
                let other_double = doubles[..i]
                    .iter()
                    .filter_map(|double| *double)
                    .any(|character| c != character);
                if other_double {
                    return true;
                }
                doubles[i] = Some(c);
            }
        }
    }
    return false;
}

fn valid(password: &[char]) -> bool {
    good_letters(password) && increasing_run(password) && double_doubles(password)
}

#[test]
fn part1() {
    let mut password: Vec<_> = "hxbxwxba".chars().rev().collect();
    while !valid(&password) {
        notch(&mut password);
    }
    let ans: String = password.iter().rev().collect();
    println!("Day 11, part 1: {}", ans);
    assert_eq!("hxbxxyzz", ans);
}

#[test]
fn part2() {
    let mut password = "hxbxxyzz".chars().rev().collect_vec();
    notch(&mut password);
    while !valid(&password) {
        notch(&mut password);
    }
    let ans: String = password.iter().rev().collect();
    println!("Day 11, part 2: {}", ans);
    assert_eq!("hxcaabcc", ans);
}
