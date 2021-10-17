const INPUT: &str = include_str!("res/06.txt");

#[derive(Debug)]
struct Point(usize, usize);

impl Point {
    fn parse(coordinate_pair: &str) -> Point {
        let (x, y) = coordinate_pair.split_once(',').unwrap();
        Point(x.parse().unwrap(), y.parse().unwrap())
    }
}

#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    first: Point,
    second: Point,
    action: Action,
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let mut tokens = line.split_ascii_whitespace();
        let action = match tokens.next() {
            Some("toggle") => Action::Toggle,
            Some("turn") => match tokens.next() {
                Some("on") => Action::On,
                Some("off") => Action::Off,
                _ => panic!("invalid input"),
            },
            _ => panic!("invalid input"),
        };
        let first = Point::parse(tokens.next().unwrap());
        tokens.next();
        let second = Point::parse(tokens.next().unwrap());
        Instruction {
            first,
            second,
            action,
        }
    }
}

#[derive(Debug)]
struct Board<const N: usize>([[bool; N]; N]);

impl<const N: usize> Board<N> {
    fn new() -> Board<N> {
        Board([[false; N]; N])
    }

    fn update(&mut self, instr: &Instruction) {
        for i in instr.first.0..=instr.second.0 {
            for j in instr.first.1..=instr.second.1 {
                match instr.action {
                    Action::On => self.0[i][j] = true,
                    Action::Off => self.0[i][j] = false,
                    Action::Toggle => self.0[i][j] = !self.0[i][j],
                }
            }
        }
    }

    fn lit(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
    }
}

// has to be a vec or the app has a stack overflow. I'm guessing this has
// something to do with the default stack size??
#[derive(Debug)]
struct FineBoard<const N: usize>(Vec<[u64; N]>);

impl<const N: usize> FineBoard<N> {
    fn new() -> FineBoard<N> {
        FineBoard(vec![[0; N]; N])
    }

    fn update(&mut self, instr: &Instruction) {
        for i in instr.first.0..=instr.second.0 {
            for j in instr.first.1..=instr.second.1 {
                match instr.action {
                    Action::On => self.0[i][j] += 1,
                    Action::Off => {
                        if self.0[i][j] > 0 {
                            self.0[i][j] -= 1;
                        }
                    }
                    Action::Toggle => self.0[i][j] += 2,
                }
            }
        }
    }

    fn brightness(&self) -> u64 {
        self.0.iter().map(|row| row.iter().sum::<u64>()).sum()
    }
}

#[test]
fn part1() {
    let instructions: Vec<_> = INPUT.lines().map(Instruction::parse).collect();
    let mut board = Board::<1000>::new();
    for instr in instructions {
        board.update(&instr)
    }
    println!("Day 6, part 1: {}", board.lit());
}

#[test]
fn part2() {
    let instructions: Vec<_> = INPUT.lines().map(Instruction::parse).collect();
    let mut board = FineBoard::<1000>::new();
    for instr in instructions {
        board.update(&instr)
    }
    println!("Day 6, part 2: {}", board.brightness());
}
