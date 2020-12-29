use crate::days::utils::Part;
use bytes::Bytes;

use futures::stream::StreamExt;
use std::{collections::HashSet, iter::FromIterator, str, usize};
use tokio::io::Result;
use tokio::{fs::File, io::BufReader};
use tokio_util::{
    codec::{FramedRead, LinesCodec},
    io::StreamReader,
};
const FILEPATH: &str = "./input/day06.txt";

pub async fn run(p: Part) -> Result<usize> {
    match p {
        Part::One => part_01().await,
        Part::Two => part_02().await,
    }
}

async fn part_01() -> Result<usize> {
    let reader = BufReader::new(File::open(FILEPATH).await?);
    // lines codec on the raw lines, replace empty lines with newline
    let rawframe = FramedRead::new(reader, LinesCodec::new());
    let reframe = rawframe.map(|l| {
        let line = l.unwrap();
        let outline = match line.len() {
            0 => "\n".to_string(),
            _ => line,
        };
        Result::Ok(Bytes::from(outline))
    });
    let datastream = FramedRead::new(StreamReader::new(reframe), LinesCodec::new());

    let tmp = datastream.fold(0, |acc, l| async move {
        let line = l.unwrap();
        let ans: std::collections::HashSet<char> = HashSet::from_iter(line.chars());
        acc + ans.len()
    });
    // get a lines codec on the raw input, when the length of the line is greater than zero, eat the newline
    // when the length of the line is equal to 0, replace it with a newline
    // put a stream_reader on that and run another lines codec, the lines should be compressed

    return Ok(tmp.await);
}

async fn part_02() -> Result<usize> {
    let reader = BufReader::new(File::open(FILEPATH).await?);
    // lines codec on the raw lines, replace empty lines with newline
    let rawframe = FramedRead::new(reader, LinesCodec::new());
    let reframe = rawframe.map(|l| {
        let line = l.unwrap();
        let outline = match line.len() {
            0 => "\n".to_string(),
            _ => line + "|",
        };
        Result::Ok(Bytes::from(outline))
    });
    let datastream = FramedRead::new(StreamReader::new(reframe), LinesCodec::new());

    let tmp = datastream.fold(0, |acc, l| async move {
        let line = l.unwrap();
        let ans = line
            .split('|')
            .filter(|gp| !gp.is_empty())
            .map(|x| HashSet::<char>::from_iter(x.chars()))
            .fold(HashSet::<char>::from_iter('a'..='z'), |mut a, b| {
                a.retain(|z| b.contains(&z));
                a
            }); //.fold_first(|a,b| )

        acc + ans.len()
    });
    // get a lines codec on the raw input, when the length of the line is greater than zero, eat the newline
    // when the length of the line is equal to 0, replace it with a newline
    // put a stream_reader on that and run another lines codec, the lines should be compressed

    return Ok(tmp.await);
}
