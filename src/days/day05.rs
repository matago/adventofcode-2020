use crate::days::utils::Part;
use futures::stream::StreamExt;
use std::ops::RangeInclusive;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader, Result},
};

const FILEPATH: &str = "./input/day05.txt";

pub async fn run(p: Part) -> Result<usize> {
    match p {
        Part::One => part_01().await,
        Part::Two => part_02().await,
    }
}

#[derive(Debug)]
struct BoardingPass<'a> {
    row: &'a str,
    col: &'a str,
}

impl<'a> BoardingPass<'a> {
    pub fn from_str(s: &str) -> BoardingPass {
        assert_eq!(s.len(), 10);
        assert!(&s[..7].chars().all(|x| matches!(x, 'F' | 'B')));
        assert!(&s[7..].chars().all(|x| matches!(x, 'L' | 'R')));
        BoardingPass {
            row: &s[..7],
            col: &s[7..],
        }
    }

    pub fn row_number(&self) -> RangeInclusive<i32> {
        self.row.chars().fold(0..=127, |rng, x| {
            let mid = 1 + (rng.end() - rng.start()) / 2;
            match x {
                'B' => *rng.start() + mid..=*rng.end(),
                'F' => *rng.start()..=*rng.end() - mid,
                _ => panic!("unexpected character found in row"),
            }
        })
    }

    pub fn col_number(&self) -> RangeInclusive<i32> {
        self.col.chars().fold(0..=7, |rng, x| {
            let mid = 1 + (rng.end() - rng.start()) / 2;
            match x {
                'R' => *rng.start() + mid..=*rng.end(),
                'L' => *rng.start()..=*rng.end() - mid,
                _ => panic!("unexpected character found in col"),
            }
        })
    }

    pub fn seat_id(&self) -> i32 {
        (self.row_number().start() * 8) + self.col_number().start()
    }
}

async fn part_01() -> Result<usize> {
    let reader = BufReader::new(File::open(FILEPATH).await?);
    let lines = reader.lines();

    let tmp = lines.fold(0, |acc, l| async move {
        let line = l.unwrap();
        let bp = BoardingPass::from_str(&line);
        if bp.seat_id() > acc {
            bp.seat_id()
        } else {
            acc
        }
    });

    return Ok(tmp.await as usize);
}

async fn part_02() -> Result<usize> {
    let reader = BufReader::new(File::open(FILEPATH).await?);
    // max 913, min 32
    let lines = reader.lines();
    let seatsum: i32 = (32..=913).sum();

    let tmp = lines.fold(seatsum, |acc, l| async move {
        let line = l.unwrap();
        let bp = BoardingPass::from_str(&line);
        acc - bp.seat_id()
    });

    return Ok(tmp.await as usize);
}
