use futures::stream::StreamExt;
use tokio::io::AsyncBufReadExt;
use tokio::{fs::File, io::BufReader};

pub enum Part {
    One,
    Two,
}

pub async fn run(p: Part) -> Result<isize, std::io::Error> {
    match p {
        Part::One => part_01().await,
        Part::Two => part_02().await,
    }
}

async fn part_01() -> Result<isize, std::io::Error> {
    let reader = BufReader::new(File::open("input/day01.txt").await?);
    let mut lines = reader.lines();
    let mut candidates: Vec<isize> = Vec::new();

    while let Some(line) = lines.next().await {
        let val: isize = line.unwrap().parse().unwrap();
        let delta = 2020 - val;

        if candidates.contains(&val) {
            return Ok(val * delta);
        }
        candidates.push(delta);
    }
    panic!("Reached EOF and no solution found")
}

pub async fn part_02() -> Result<isize, std::io::Error> {
    let reader = BufReader::new(File::open("input/day01.txt").await?);
    let mut lines = reader.lines();
    let mut candidates: Vec<isize> = Vec::new();

    while let Some(line) = lines.next().await {
        let val: isize = line.unwrap().parse().unwrap();

        for cval in candidates.iter() {
            let delta = 2020 - val - cval;
            if delta > 0 && candidates.contains(&delta) {
                return Ok(val * cval * delta);
            }
        }
        candidates.push(val);
    }
    panic!("Reached EOF and no solution found")
}
