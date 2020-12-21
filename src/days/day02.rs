use futures::stream::StreamExt;
use recap::Recap;
use serde::Deserialize;
use tokio::io::AsyncBufReadExt;
use tokio::{fs::File, io::BufReader};

pub enum Part {
    One,
    Two,
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?x)
    (?P<min>\d+)
    -
    (?P<max>\d+)
    \s+
    (?P<keychar>\S+)
    :\s+
    (?P<password>\S+)
  "#)]
struct Entry {
    min: usize,
    max: usize,
    keychar: char,
    password: String,
}

impl Entry {
    fn is_valid(&self) -> bool {
        match self.password.matches(self.keychar).count() {
            x if x < self.min => false,
            x if x > self.max => false,
            _ => true,
        }
    }

    fn still_valid(&self) -> bool {
        let mut tmp = self.password.chars();
        match (
            tmp.nth(self.min - 1) == Some(self.keychar),
            tmp.nth(self.max - 1 - self.min) == Some(self.keychar),
        ) {
            (true, false) => true,
            (false, true) => true,
            _ => false,
        }
    }
}
pub async fn run(p: Part) -> Result<isize, std::io::Error> {
    match p {
        Part::One => part_01().await,
        Part::Two => part_02().await,
    }
}

async fn part_01() -> Result<isize, std::io::Error> {
    let reader = BufReader::new(File::open("input/day02.txt").await?);
    let lines = reader.lines();

    let result: isize = lines
        .fold(0, |acc, line| async move {
            let entry: Entry = line.unwrap().parse().unwrap();
            match entry.is_valid() {
                true => acc + 1,
                false => acc,
            }
        })
        .await;
    return Ok(result);
}

async fn part_02() -> Result<isize, std::io::Error> {
    let reader = BufReader::new(File::open("input/day02.txt").await?);
    let lines = reader.lines();

    let result: isize = lines
        .fold(0, |acc, line| async move {
            let entry: Entry = line.unwrap().parse().unwrap();
            match entry.still_valid() {
                true => acc + 1,
                false => acc,
            }
        })
        .await;
    return Ok(result);
}
