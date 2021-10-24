use itertools::Itertools;

const INPUT: &str = include_str!("res/14.txt");

#[derive(Debug)]
struct Reindeer {
    flight_speed: u32,
    flight_duration: u32,
    rest_duration: u32,
    points: u32,
}

impl Reindeer {
    fn parse(line: &str) -> Reindeer {
        // Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.
        let mut tokens = line.split_ascii_whitespace();
        let flight_speed = tokens.nth(3).unwrap().parse().unwrap();
        let flight_duration = tokens.nth(2).unwrap().parse().unwrap();
        let rest_duration = tokens.nth(6).unwrap().parse().unwrap();
        Reindeer {
            flight_speed,
            flight_duration,
            rest_duration,
            points: 0,
        }
    }

    fn period(&self) -> u32 {
        self.flight_duration + self.rest_duration
    }

    fn distance_per_period(&self) -> u32 {
        self.flight_speed * self.flight_duration
    }

    fn position_at(&self, time: u32) -> u32 {
        let position_in_period = time % self.period();
        let last_leg = if position_in_period < self.flight_duration {
            self.flight_speed * position_in_period
        } else {
            self.distance_per_period()
        };
        let distance_over_completed_periods = self.distance_per_period() * (time / self.period());
        distance_over_completed_periods + last_leg
    }

    fn award_point(&mut self) {
        self.points += 1;
    }
}

#[test]
fn part1() {
    let ans = INPUT
        .lines()
        .map(Reindeer::parse)
        .map(|r| r.position_at(2503))
        .max()
        .unwrap();
    println!("Day 14, part 1: {}", ans);
}

#[test]
fn part2() {
    let mut reindeer = INPUT.lines().map(Reindeer::parse).collect_vec();
    for i in 1..2503 {
        let lead = reindeer.iter().map(|r| r.position_at(i)).max().unwrap();
        reindeer
            .iter_mut()
            .filter(|r| r.position_at(i) == lead)
            .for_each(|r| r.award_point());
    }
    let ans = reindeer.iter().map(|r| r.points).max().unwrap();
    println!("Day 14, part 2: {}", ans);
}
