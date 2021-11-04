use std::collections::{hash_map::Entry, HashMap, HashSet};

use itertools::Itertools;

const RULES: &str = include_str!("res/19.txt");
const INPUT: &str = "CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr";

#[derive(Debug)]
struct Rules(HashMap<Vec<char>, Vec<Vec<char>>>);

fn find_all(pattern: &[char], text: &[char]) -> Vec<usize> {
    let mut occurrences = vec![];
    text.windows(pattern.len())
        .enumerate()
        .for_each(|(i, chars)| {
            if chars == pattern {
                occurrences.push(i);
            }
        });
    occurrences
}

impl Rules {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Rules {
        let mut rules: HashMap<_, Vec<_>> = HashMap::new();
        for line in lines {
            let (before, after) = line.split_once(" => ").unwrap();
            let (before, after) = (before.chars().collect_vec(), after.chars().collect_vec());
            match rules.entry(before) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(after);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![after]);
                }
            }
        }
        Rules(rules)
    }

    fn permutations(&self, input: &str) -> HashSet<String> {
        let mut perms = vec![];
        let input = input.chars().collect_vec();
        for (key, replacements) in &self.0 {
            perms.append(&mut Rules::sub_all(&input, &key, &replacements));
        }
        perms.into_iter().collect()
    }

    fn sub_all(text: &[char], before: &[char], replacements: &[Vec<char>]) -> Vec<String> {
        replacements
            .iter()
            .map(|replacement| Rules::sub(text, before, replacement))
            .flat_map(|v| v.into_iter())
            .collect_vec()
    }

    fn sub(text: &[char], before: &[char], after: &[char]) -> Vec<String> {
        find_all(before, text)
            .iter()
            .map(|i| {
                let mut new = text[..*i].iter().collect_vec();
                let mut pat = after.iter().collect_vec();
                new.append(&mut pat);
                new.append(&mut text[i + before.len()..].iter().collect_vec());
                new.iter().map(|c| *c).collect()
            })
            .collect_vec()
    }
}

#[test]
fn part1() {
    let rules = Rules::parse(RULES.lines());
    let perms = rules.permutations(INPUT);
    println!("Day 19, part 1: {}", perms.len());
    assert_eq!(509, perms.len());
}
