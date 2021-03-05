mod days;
use days::{day01, day02, day03, day04, day05, day06, day07, day08, utils::Part};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Days {
    All,
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
}
#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    day: Days,
}

async fn runner(day: &Days) -> () {
    match &day {
        Days::All => {
            println!("If this prints something went wrong");
        }
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
        Days::Day06 => {
            println!("Day 6.1: {}", day06::run(Part::One).await.unwrap());
            println!("Day 6.2: {}", day06::run(Part::Two).await.unwrap())
        }
        Days::Day07 => {
            println!("Day 7.1: {}", day07::run(Part::One).await.unwrap());
            println!("Day 7.2: {}", day07::run(Part::Two).await.unwrap())
        }
        Days::Day08 => {
            println!("Day 8.1: {}", day08::run(Part::One).await.unwrap());
            println!("Day 8.2: {}", day08::run(Part::Two).await.unwrap())
        }
    }
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    match &opt.day {
        Days::All => {
            for day in &[
                Days::Day01,
                Days::Day02,
                Days::Day03,
                Days::Day04,
                Days::Day05,
                Days::Day06,
                Days::Day07,
                Days::Day08,
            ] {
                runner(day).await;
            }
        }
        _ => {
            runner(&opt.day).await;
        }
    }
}
