use crate::days::utils::Part;

use futures::stream::StreamExt;
use std::{
    collections::HashSet,
    convert::TryInto,
    fmt::Debug,
    str::{self, FromStr},
    usize,
};
use tokio::io::{AsyncBufReadExt, Result};
use tokio::{fs::File, io::BufReader};

const FILEPATH: &str = "./input/day08.txt";

pub async fn run(p: Part) -> Result<usize> {
    match p {
        Part::One => part_01().await,
        Part::Two => part_02().await,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Command {
    operation: Operation,
    operand: isize,
}

#[derive(Debug, Clone, Copy, Default)]
struct Program {
    index: usize,
    accumulator: isize,
}

type Instructions = Vec<Command>;

fn from_lines(lines: Vec<String>) -> Instructions {
    return lines
        .into_iter()
        .map(|l| Command::from_str(&l.to_string()).unwrap())
        .collect();
}

impl Program {
    fn next(&mut self, instructions: &Instructions) -> Option<bool> {
        match instructions.get(self.index) {
            Some(command) => {
                match command.operation {
                    Operation::Nop => {
                        self.index += 1;
                    }
                    Operation::Acc => {
                        self.index += 1;
                        self.accumulator += command.operand;
                    }
                    Operation::Jmp => {
                        self.index = (self.index as isize + command.operand).try_into().unwrap();
                    }
                }
                return Some(true);
            }
            None => return None,
        }
    }
}

impl Command {
    fn flip(&mut self) -> bool {
        match self.operation {
            Operation::Jmp => {
                self.operation = Operation::Nop;
                return true;
            }
            Operation::Nop => {
                self.operation = Operation::Jmp;
                return true;
            }
            _ => return false,
        }
    }
}

impl str::FromStr for Command {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        Ok(Command {
            operation: match tokens.next() {
                Some(token) => match token {
                    "nop" => Operation::Nop,
                    "acc" => Operation::Acc,
                    "jmp" => Operation::Jmp,
                    _ => panic!("not a valid operation"),
                },
                None => panic!("no operation provided"),
            },
            operand: match tokens.next() {
                Some(token) => token.parse().unwrap(),
                None => panic!("not a valid operand"),
            },
        })
    }
}

async fn part_01() -> Result<usize> {
    let reader = BufReader::new(File::open(FILEPATH).await?);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect().await;

    let tapes = from_lines(lines);

    let mut program: Program = Program::default();
    let mut history: HashSet<usize> = HashSet::new();

    while history.insert(program.index) {
        program.next(&tapes);
    }

    Ok(program.accumulator.try_into().unwrap())
}

async fn part_02() -> Result<usize> {
    let reader = BufReader::new(File::open(FILEPATH).await?);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect().await;

    let mut tapes = from_lines(lines);

    for i in 0..tapes.len() {
        let mut program: Program = Program::default();
        let mut history: HashSet<usize> = HashSet::new();

        let mut cmd = tapes[i];

        if cmd.flip() {
            tapes[i] = cmd;
            while history.insert(program.index) {
                if let Some(_) = program.next(&tapes) {
                } else {
                    println!("Program terminated on None");
                    return Ok(program.accumulator.try_into().unwrap());
                }
            }
            cmd.flip();
            tapes[i] = cmd;
        }
    }
    Ok(0)
}
