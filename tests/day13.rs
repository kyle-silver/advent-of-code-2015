use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("res/13.txt");

#[derive(Debug)]
struct FamilyOpinions<'a>(HashMap<(&'a str, &'a str), i32>);

impl<'a> FamilyOpinions<'a> {
    fn parse(lines: impl Iterator<Item = &'a str>) -> FamilyOpinions<'a> {
        let mut map = HashMap::new();
        for line in lines {
            let mut tokens = line.split_ascii_whitespace();
            // let primary = tokens.next().unwrap();
            let primary = tokens.nth(0).unwrap();
            let multiplier = if let Some("lose") = tokens.nth(1) {
                -1
            } else {
                1
            };
            let amount = multiplier * tokens.nth(0).unwrap().parse::<i32>().unwrap();
            let secondary = tokens.nth(6).unwrap();
            map.insert((primary, secondary), amount);
        }
        FamilyOpinions(map)
    }

    fn insert_true_neutral(&mut self, name: &'a str) {
        let distinct: HashSet<_> = self.0.keys().map(|(primary, _)| *primary).collect();
        for member in distinct {
            self.0.insert((name, member), 0);
            self.0.insert((member, name), 0);
        }
    }

    fn members(&self) -> Vec<&str> {
        let distinct: HashSet<_> = self.0.keys().map(|(primary, _)| *primary).collect();
        return distinct.into_iter().collect_vec();
    }

    fn score(&self, a: &str, b: &str) -> i32 {
        self.0.get(&(a, b)).unwrap() + self.0.get(&(b, a)).unwrap()
    }

    fn total_score(&self, arrangement: &[&&str]) -> i32 {
        arrangement
            .iter()
            .circular_tuple_windows()
            .map(|(&&a, &&b)| self.score(a, b))
            .sum()
    }
}

#[test]
fn part1() {
    let opinions = FamilyOpinions::parse(INPUT.lines());
    let members = opinions.members();
    let ans = members
        .iter()
        .permutations(members.len())
        .map(|arrangement| opinions.total_score(&arrangement))
        .max()
        .unwrap();
    println!("Day 13, part 1: {}", ans);
}

#[test]
fn part2() {
    let mut opinions = FamilyOpinions::parse(INPUT.lines());
    opinions.insert_true_neutral("me");
    let members = opinions.members();
    let ans = members
        .iter()
        .permutations(members.len())
        .map(|arrangement| opinions.total_score(&arrangement))
        .max()
        .unwrap();
    println!("Day 13, part 1: {}", ans);
}
