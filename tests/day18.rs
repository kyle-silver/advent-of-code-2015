const INPUT: &str = include_str!("res/18.txt");

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct Life<const N: usize>([[bool; N]; N]);

impl<const N: usize> Life<N> {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Life<N> {
        let mut grid = [[false; N]; N];
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid[i][j] = match c {
                    '#' => true,
                    _ => false,
                };
            }
        }
        Life(grid)
    }

    fn next(&self) -> Life<N> {
        let mut new = [[false; N]; N];
        for i in 0..N {
            for j in 0..N {
                let neighbors = self.neighbors(i, j);
                if self.0[i][j] {
                    new[i][j] = neighbors == 2 || neighbors == 3;
                } else {
                    new[i][j] = neighbors == 3;
                }
            }
        }
        Life(new)
    }

    fn neighbors(&self, i: usize, j: usize) -> usize {
        return NEIGHBORS
            .iter()
            .map(|(di, dj)| {
                if i as isize + di >= N as isize || i as isize + di < 0 {
                    return false;
                }
                if j as isize + dj >= N as isize || j as isize + dj < 0 {
                    return false;
                }
                self.0[(i as isize + di) as usize][(j as isize + dj) as usize]
            })
            .filter(|b| *b)
            .count();
    }

    fn alive(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }

    fn peg_corners(&mut self) {
        self.0[0][0] = true;
        self.0[0][N - 1] = true;
        self.0[N - 1][0] = true;
        self.0[N - 1][N - 1] = true;
    }
}

#[test]
fn part1() {
    let mut life = Life::<100>::parse(INPUT.lines());
    for _ in 0..100 {
        life = life.next();
    }
    println!("Day 18, part 1: {}", life.alive());
}

#[test]
fn part2() {
    let mut life = Life::<100>::parse(INPUT.lines());
    life.peg_corners();
    for _ in 0..100 {
        life = life.next();
        life.peg_corners();
    }
    println!("Day 18, part 2: {}", life.alive());
}
