use std::collections::HashSet;

const INPUT: &str = include_str!("res/03.txt");

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn parse(dir: char) -> Direction {
        use Direction::*;
        match dir {
            '^' => North,
            'v' => South,
            '>' => East,
            '<' => West,
            _ => panic!(),
        }
    }

    fn next(&self, current: (i32, i32)) -> (i32, i32) {
        use Direction::*;
        match self {
            North => (current.0, current.1 + 1),
            South => (current.0, current.1 - 1),
            East => (current.0 + 1, current.1),
            West => (current.0 - 1, current.1),
        }
    }
}

#[test]
fn part1() {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut pos = (0, 0);
    for dir in INPUT.chars().map(Direction::parse) {
        pos = dir.next(pos);
        visited.insert(pos);
    }
    println!("Day 3, part 1: {}", visited.len());
}

#[test]
fn part2() {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    for (i, dir) in INPUT.chars().map(Direction::parse).enumerate() {
        if i % 2 == 0 {
            pos1 = dir.next(pos1);
            visited.insert(pos1);
        } else {
            pos2 = dir.next(pos2);
            visited.insert(pos2);
        }
    }
    println!("Day 3, part 2: {}", visited.len());
}
