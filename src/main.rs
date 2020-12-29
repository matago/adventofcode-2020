mod days;
use days::{day01, day02, day03, day04, day05, utils::Part};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Days {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
}
#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    day: Days,
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    match opt.day {
        Days::Day01 => {
            println!("Day 1.1: {}", day01::run(Part::One).await.unwrap());
            println!("Day 1.2: {}", day01::run(Part::Two).await.unwrap());
        }
        Days::Day02 => {
            println!("Day 2.1: {}", day02::run(Part::One).await.unwrap());
            println!("Day 2.2: {}", day02::run(Part::Two).await.unwrap())
        }
        Days::Day03 => {
            println!("Day 3.1: {}", day03::run(Part::One).await.unwrap());
            println!("Day 3.2: {}", day03::run(Part::Two).await.unwrap())
        }
        Days::Day04 => {
            println!("Day 4.1: {}", day04::run(Part::One).await.unwrap());
            println!("Day 4.2: {}", day04::run(Part::Two).await.unwrap())
        }
        Days::Day05 => {
            println!("Day 5.1: {}", day05::run(Part::One).await.unwrap());
            println!("Day 5.2: {}", day05::run(Part::Two).await.unwrap())
        }
    }
}
