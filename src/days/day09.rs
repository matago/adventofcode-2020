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

const FILEPATH: &str = "./input/day09.txt";

pub async fn run(p: Part) -> Result<usize> {
    match p {
        Part::One => part_01().await,
        Part::Two => part_02().await,
    }
}

async fn part_01() -> Result<usize> {
    let reader = BufReader::new(File::open(FILEPATH).await?);

    let vals: Vec<usize> = reader
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect()
        .await;

    let result = vals
        .windows(26)
        .find(|window| {
            for i in 0..24 {
                for j in (i + 1)..25 {
                    if window[i] + window[j] == window[25] {
                        return false;
                    }
                }
            }
            return true;
        })
        .unwrap()[25];

    Ok(result)
}

async fn part_02() -> Result<usize> {
    let reader = BufReader::new(File::open(FILEPATH).await?);

    let FIND: usize = 556543474;

    let vals: Vec<usize> = reader
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect()
        .await;

    let mut head: usize = 1;
    let mut tail: usize = 0;

    let mut total: usize = vals[tail] + vals[head];

    while total != FIND {
        while total < FIND {
            head += 1;
            total += vals[head];
        }

        while total > FIND {
            total -= vals[tail];
            tail += 1;
        }
    }

    let result =
        &vals[tail..head]
            .iter()
            .fold(
                (usize::MAX as usize, usize::MIN as usize),
                |acc, v| match v < &acc.0 {
                    true => (*v, acc.1),
                    false => match v > &acc.1 {
                        true => (acc.0, *v),
                        false => acc,
                    },
                },
            );

    Ok(result.0 + result.1)
}
