use crate::days::utils::Part;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{str, usize};
use tokio::io::Result;
use tokio::stream::StreamExt;
use tokio::{fs::File, io::BufReader};
use tokio_util::codec::{FramedRead, LinesCodec};
use tokio_util::io::StreamReader;
const FILEPATH: &str = "../input/day04.txt";

pub async fn run(p: Part) -> Result<usize> {
    match p {
        Part::One => part_01().await,
        Part::Two => part_01().await,
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Passport {
    ecl: Option<String>,
    pid: Option<String>,
    eyr: Option<String>,
    hcl: Option<String>,
    byr: Option<String>,
    iyr: Option<String>,
    cid: Option<String>,
    hgt: Option<String>,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        !(self.ecl.is_none()
            || self.pid.is_none()
            || self.eyr.is_none()
            || self.hcl.is_none()
            || self.byr.is_none()
            || self.iyr.is_none()
            || self.hgt.is_none())
    }

    pub fn valid_byr(self) -> bool {
        match self.byr {
            Some(v) => matches!(v.parse::<usize>(), Ok(1920..=2002)),
            _ => false,
        }
    }

    pub fn valid_iyr(self) -> bool {
        match self.iyr {
            Some(v) => matches!(v.parse::<usize>(), Ok(2010..=2020)),
            _ => false,
        }
    }

    pub fn valid_eyr(self) -> bool {
        match self.eyr {
            Some(v) => matches!(v.parse::<usize>(), Ok(2020..=2030)),
            _ => false,
        }
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
            _ => line + " ",
        };
        Result::Ok(Bytes::from(outline))
    });
    let datastream = FramedRead::new(StreamReader::new(reframe), LinesCodec::new());

    let tmp: usize = datastream
        .fold(0, |acc, l| {
            let line = l.unwrap();
            // deal with yaml coersion bullshit
            let line = line
                .replace("#", "\\#")
                .replace(" ", "\n")
                .replace(":", ": ");
            let passport: Passport = serde_yaml::from_str(&line[..]).unwrap();
            if passport.is_valid() {
                println!("{:?}", passport);
                acc + 1
            } else {
                acc
            }
        })
        .await;
    // get a lines codec on the raw input, when the length of the line is greater than zero, eat the newline
    // when the length of the line is equal to 0, replace it with a newline
    // put a stream_reader on that and run another lines codec, the lines should be compressed

    return Ok(tmp);
}
