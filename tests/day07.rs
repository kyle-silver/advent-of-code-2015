use std::{
    collections::{HashMap, HashSet},
    ops::{Shl, Shr},
};

const INPUT: &str = include_str!("res/07.txt");

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Arg {
    Literal(u16),
    Register(usize),
}

impl Arg {
    fn parse(arg: &str) -> Arg {
        if let Ok(literal) = arg.parse() {
            Arg::Literal(literal)
        } else {
            Arg::Register(Arg::reg_addr(arg))
        }
    }

    /// converting the registers from names to usize values keeps us from
    /// needing to use a hashmap and having `String`s flying around everywhere
    fn reg_addr(reg_name: &str) -> usize {
        let mut reg_addr: usize = 0;
        for (i, c) in reg_name.chars().rev().enumerate() {
            let alphabetic_position = (c as u8 - 'a' as u8 + 1) as usize;
            reg_addr += alphabetic_position * 27usize.pow(i as u32);
        }
        reg_addr
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Op {
    Write(Arg),
    And(Arg, Arg),
    Or(Arg, Arg),
    Not(Arg),
    LeftShift { reg: Arg, bits: Arg },
    RightShift { reg: Arg, bits: Arg },
}

impl Op {
    fn parse(lhs: &str) -> Op {
        use Op::*;
        let mut tokens = lhs.split_ascii_whitespace();
        let first = tokens.next().unwrap();
        if first.eq("NOT") {
            let reg = Arg::parse(tokens.next().unwrap());
            return Not(reg);
        }
        let arg0 = Arg::parse(first);
        let instr = match tokens.next() {
            Some(instruction) => instruction,
            None => return Write(arg0),
        };
        let arg1 = Arg::parse(tokens.next().unwrap());
        match instr {
            "AND" => And(arg0, arg1),
            "OR" => Or(arg0, arg1),
            "LSHIFT" => LeftShift {
                reg: arg0,
                bits: arg1,
            },
            "RSHIFT" => RightShift {
                reg: arg0,
                bits: arg1,
            },
            _ => panic!("Invalid op type"),
        }
    }

    fn dependencies(&self) -> Vec<&Arg> {
        match self {
            Op::Write(arg) => vec![arg],
            Op::And(a1, a2) => vec![a1, a2],
            Op::Or(a1, a2) => vec![a1, a2],
            Op::Not(a) => vec![a],
            Op::LeftShift { reg, bits } => vec![reg, bits],
            Op::RightShift { reg, bits } => vec![reg, bits],
        }
    }

    fn registers_used(&self) -> Vec<usize> {
        self.dependencies()
            .into_iter()
            .filter_map(|a| match a {
                Arg::Literal(_) => None,
                Arg::Register(addr) => Some(*addr),
            })
            .collect()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Instruction {
    op: Op,
    destination: usize,
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let (op_raw, dest_raw) = input.split_once(" -> ").unwrap();
        Instruction {
            op: Op::parse(op_raw),
            destination: Arg::reg_addr(dest_raw),
        }
    }
}

#[derive(Debug)]
struct CircuitBoard([u16; 756]);

impl CircuitBoard {
    fn new() -> CircuitBoard {
        CircuitBoard([0; 756])
    }

    fn get(&self, arg: &Arg) -> u16 {
        match arg {
            Arg::Literal(value) => *value,
            Arg::Register(addr) => self.0[*addr],
        }
    }

    fn update(&mut self, instr: &Instruction) {
        let val = match &instr.op {
            Op::Write(arg) => self.get(arg),
            Op::And(a1, a2) => self.get(a1) & self.get(a2),
            Op::Or(a1, a2) => self.get(a1) | self.get(a2),
            Op::Not(arg) => !self.get(arg),
            Op::LeftShift { reg, bits } => self.get(reg).shl(self.get(bits)),
            Op::RightShift { reg, bits } => self.get(reg).shr(self.get(bits)),
        };
        self.0[instr.destination] = val;
    }
}

#[derive(Debug)]
struct DependencyGraph<'a> {
    instructions: Vec<&'a Instruction>,
}

impl DependencyGraph<'_> {
    fn new(input: &[Instruction]) -> DependencyGraph {
        let mut unused: HashSet<&Instruction> = input.iter().collect();
        let mut dependencies = HashMap::new();
        let mut instructions = Vec::new();
        while !unused.is_empty() {
            let next = unused.iter().find(|instr| {
                instr
                    .op
                    .dependencies()
                    .iter()
                    .all(|d| Self::_ready(d, &dependencies))
            });
            if let Some(&instr) = next {
                unused.remove(&instr);
                dependencies.insert(instr.destination, instr.op.registers_used());
                instructions.push(instr);
            }
        }
        DependencyGraph { instructions }
    }

    fn _ready(dep: &Arg, graph: &HashMap<usize, Vec<usize>>) -> bool {
        match dep {
            Arg::Literal(_) => true,
            Arg::Register(addr) => graph.contains_key(addr),
        }
    }
}

#[test]
fn part1() {
    // gather up the instructions
    let instructions: Vec<_> = INPUT.lines().map(Instruction::parse).collect();
    // we need to construct a dependency graph to figure out the execution
    // order... great...
    let graph = DependencyGraph::new(&instructions);
    // now we can iterate over the instructions in order and evaluate what's
    // going on
    let mut circuit_board = CircuitBoard::new();
    for instr in &graph.instructions {
        circuit_board.update(instr);
    }
    println!("Day 7, part 1: {}", circuit_board.0[Arg::reg_addr("a")]);
    assert_eq!(3176, circuit_board.0[Arg::reg_addr("a")]);
}

#[test]
fn part2() {
    let mut instructions: Vec<_> = INPUT.lines().map(Instruction::parse).collect();
    instructions[3] = Instruction {
        op: Op::Write(Arg::Literal(3176)),
        destination: Arg::reg_addr("b"),
    };
    let graph = DependencyGraph::new(&instructions);
    let mut circuit_board = CircuitBoard::new();
    for instr in &graph.instructions {
        circuit_board.update(instr);
    }
    println!("Day 7, part 2: {}", circuit_board.0[Arg::reg_addr("a")]);
    assert_eq!(14710, circuit_board.0[Arg::reg_addr("a")]);
}
