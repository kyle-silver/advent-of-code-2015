use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("res/09.txt");

#[derive(Debug)]
struct MapData<'a> {
    distances: HashMap<(&'a str, &'a str), u32>,
    locations: HashSet<&'a str>,
}

impl<'a> MapData<'a> {
    fn parse(lines: impl Iterator<Item = &'a str>) -> Option<MapData<'a>> {
        let mut locations = HashSet::new();
        let mut distances = HashMap::new();
        for line in lines {
            let mut tokens = line.split_ascii_whitespace();
            let start = tokens.next()?;
            tokens.next();
            let end = tokens.next()?;
            tokens.next();
            let distance = tokens.next()?.parse().unwrap();
            distances.insert((start, end), distance);
            distances.insert((end, start), distance);
            locations.insert(start);
            locations.insert(end);
        }
        Some(MapData {
            locations,
            distances,
        })
    }

    fn travel_distance(&self, path: &[&&str]) -> u32 {
        path.windows(2)
            .map(|pair| self.distances.get(&(pair[0], pair[1])).unwrap())
            .sum()
    }
}

#[test]
fn part1() {
    let data = MapData::parse(INPUT.lines()).unwrap();
    let ans = data
        .locations
        .iter()
        .permutations(data.locations.len())
        .unique()
        .map(|path| data.travel_distance(&path))
        .min()
        .unwrap();
    println!("Day 9, part 1: {}", ans);
}

#[test]
fn part2() {
    let data = MapData::parse(INPUT.lines()).unwrap();
    let ans = data
        .locations
        .iter()
        .permutations(data.locations.len())
        .unique()
        .map(|path| data.travel_distance(&path))
        .max()
        .unwrap();
    println!("Day 9, part 2: {}", ans);
}
