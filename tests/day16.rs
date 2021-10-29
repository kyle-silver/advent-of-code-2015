use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("res/16.txt");

#[derive(Debug)]
struct Sue<'a>(HashMap<&'a str, u32>);

impl<'a> Sue<'a> {
    fn parse(line: &'a str) -> Sue<'a> {
        let mut map = HashMap::new();
        let mut iter = line.split_ascii_whitespace();
        // skip the sue number
        iter.nth(1);
        while let Some(attr) = iter.next() {
            map.insert(attr, iter.next().unwrap().parse().unwrap());
        }
        Sue(map)
    }

    fn common_attrs(&self, other: &Self) -> u32 {
        self.0
            .iter()
            .map(|(key, value)| match other.0.get(key) {
                Some(other_value) => {
                    if value == other_value {
                        1
                    } else {
                        0
                    }
                }
                None => 0,
            })
            .sum()
    }

    fn retroencabulated_attrs(&self, other: &Self) -> u32 {
        self.0
            .iter()
            .map(|(key, value)| match other.0.get(key) {
                Some(other_value) => match *key {
                    "cats" | "trees" => {
                        if other_value >= value {
                            1
                        } else {
                            0
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if other_value <= value {
                            1
                        } else {
                            0
                        }
                    }
                    _ => {
                        if value == other_value {
                            1
                        } else {
                            0
                        }
                    }
                },
                None => 0,
            })
            .sum()
    }
}

#[test]
fn part1() {
    let aunts = INPUT.lines().map(Sue::parse).collect_vec();
    let sue = Sue(vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 5),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect());

    let ans = aunts
        .iter()
        .enumerate()
        .max_by(|(_, s1), (_, s2)| sue.common_attrs(s1).cmp(&sue.common_attrs(s2)))
        .map(|(i, _)| i + 1)
        .unwrap();
    println!("Day 16, part 1: {}", ans);
    assert_eq!(ans, 40);
}

#[test]
fn part2() {
    let aunts = INPUT.lines().map(Sue::parse).collect_vec();
    let sue = Sue(vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 5),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect());

    let ans = aunts
        .iter()
        .enumerate()
        .max_by(|(_, s1), (_, s2)| {
            sue.retroencabulated_attrs(s1)
                .cmp(&sue.retroencabulated_attrs(s2))
        })
        .map(|(i, _)| i + 1)
        .unwrap();
    println!("Day 16, part 2: {}", ans);
    assert_eq!(241, ans);
}
