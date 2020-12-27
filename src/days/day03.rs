use futures::stream::StreamExt;
use tokio::io::AsyncBufReadExt;
use tokio::{fs::File, io::BufReader};

use crate::days::utils::Part;

const FILEPATH: &str = "./input/day03.txt";

pub async fn run(p: Part) -> Result<usize, std::io::Error> {
    match p {
        Part::One => part_01().await,
        Part::Two => part_02().await,
    }
}

pub struct Nav {
    pos: (usize, usize),
    step: (usize, usize),
    hits: usize,
}

impl Nav {
    pub fn new(step: (usize, usize)) -> Nav {
        Nav {
            pos: (0, 0),
            step,
            hits: 0,
        }
    }

    fn update(&mut self) {
        self.pos.0 += self.step.0;
        self.pos.1 += self.step.1;
    }

    pub fn traverse(&mut self, row: usize, path: &str) {
        if row >= self.pos.0 {
            let landing = path.chars().cycle().nth(self.pos.1);
            if landing == Some('#') {
                self.hits += 1;
            }
            self.update();
        }
    }
}

async fn part_01() -> Result<usize, std::io::Error> {
    let reader = BufReader::new(File::open(FILEPATH).await?);
    let lines = reader.lines();

    let result: Nav = lines
        .enumerate()
        .fold(Nav::new((1, 3)), |mut acc, (i, line)| async move {
            let path = line.unwrap();
            acc.traverse(i, &path[..]);
            acc
        })
        .await;
    return Ok(result.hits);
}

async fn part_02() -> Result<usize, std::io::Error> {
    let reader = BufReader::new(File::open(FILEPATH).await?);
    let lines = reader.lines();

    let results: Vec<Nav> = lines
        .enumerate()
        .fold(
            vec![
                Nav::new((1, 1)),
                Nav::new((1, 3)),
                Nav::new((1, 5)),
                Nav::new((1, 7)),
                Nav::new((2, 1)),
            ],
            |mut acc, (i, line)| async move {
                let path = line.unwrap();
                for nav in acc.iter_mut() {
                    nav.traverse(i, &path[..])
                }
                acc
            },
        )
        .await;
    let mut tmp: usize = 1;

    for result in results {
        tmp *= result.hits
    }

    return Ok(tmp);
}
