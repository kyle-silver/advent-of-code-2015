use std::{convert::TryInto, str::FromStr};

const INPUT: &str = include_str!("res/02.txt");

#[derive(Debug)]
struct Present(u32, u32, u32);

impl Present {
    fn parse(token: &str) -> Present {
        let dimensions: Vec<_> = token
            .split('x')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        Present(dimensions[0], dimensions[1], dimensions[2])
    }

    fn surface_area_with_slack(self) -> u32 {
        let faces = [self.0 * self.1, self.0 * self.2, self.1 * self.2];
        let surface_area = 2 * faces.iter().sum::<u32>();
        let slack = faces.iter().min().unwrap();
        return surface_area + slack;
    }

    fn ribbon_length(self) -> u32 {
        let perimeters = [
            2 * (self.0 + self.1),
            2 * (self.0 + self.2),
            2 * (self.1 + self.2),
        ];
        let ribbon = perimeters.iter().min().unwrap();
        let bow = self.0 * self.1 * self.2;
        return ribbon + bow;
    }
}

#[test]
fn part1() {
    let ans: u32 = INPUT
        .lines()
        .map(Present::parse)
        .map(Present::surface_area_with_slack)
        .sum();
    println!("Day 2, part 1: {}", ans);
}

#[test]
fn part2() {
    let ans: u32 = INPUT
        .lines()
        .map(Present::parse)
        .map(Present::ribbon_length)
        .sum();
    println!("Day 2, part 2: {}", ans);
}
