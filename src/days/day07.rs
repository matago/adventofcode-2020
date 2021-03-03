use crate::days::utils::Part;

use futures::stream::StreamExt;
use regex::Regex;
use std::{collections::HashMap, str, usize};
use tokio::io::Result;
use tokio::{fs::File, io::BufReader};
use tokio_util::codec::{FramedRead, LinesCodec};

const FILEPATH: &str = "./input/day07.txt";

pub async fn run(p: Part) -> Result<usize> {
    match p {
        Part::One => part_01().await,
        Part::Two => part_02().await,
    }
}

async fn gen_bag() -> HashMap<String, Vec<(usize, String)>> {
    let reader = BufReader::new(File::open(FILEPATH).await.unwrap());
    // lines codec on the raw lines, replace empty lines with newline
    let rawframe = FramedRead::new(reader, LinesCodec::new());

    let re_line: Regex = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();
    let re_items: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

    let mut bags = HashMap::<String, Vec<(usize, String)>>::new();
    let _ = rawframe
        .map(|l| {
            let line = l.unwrap();
            if let Some((parent, remainder)) = re_line
                .captures(&line)
                .and_then(|capture| Some((capture.get(1)?.as_str(), capture.get(2)?.as_str())))
            {
                bags.insert(
                    parent.to_string(),
                    re_items
                        .captures_iter(remainder)
                        .filter_map(|capture| {
                            Some((
                                capture.get(1)?.as_str().parse().ok()?,
                                capture.get(2)?.as_str().to_string(),
                            ))
                        })
                        .collect(),
                );
            }
            0 as usize;
        })
        .collect::<Vec<_>>()
        .await;
    return bags;
}

fn has_child(graph: &HashMap<String, Vec<(usize, String)>>, key: &String, bag_name: &str) -> bool {
    if let Some(children) = graph.get(key) {
        for (_, child) in children {
            if child == bag_name {
                return true;
            } else if has_child(graph, child, bag_name) {
                return true;
            }
        }
        false
    } else {
        false
    }
}

fn child_bags(graph: &HashMap<String, Vec<(usize, String)>>, key: &String) -> usize {
    let mut total: usize = 0;
    if let Some(children) = graph.get(key) {
        for (n, child) in children {
            total += n + (n * child_bags(graph, child));
        }
    }
    return total;
}

async fn part_01() -> Result<usize> {
    let bags = gen_bag().await;

    let z = bags
        .keys()
        .filter(|key| has_child(&bags, key, &"shiny gold"))
        .count();

    Ok(z)
}

async fn part_02() -> Result<usize> {
    let bags = gen_bag().await;

    Ok(child_bags(&bags, &"shiny gold".to_string()))
}
