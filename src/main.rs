use structopt::StructOpt;

mod days;
use days::day01;

#[derive(StructOpt, Debug)]
enum Days {
    Day01,
    Day02,
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
            println!(
                "Day 1, part 1: {}",
                day01::run(day01::Part::One).await.unwrap()
            );
            println!(
                "Day 1, part 2: {}",
                day01::run(day01::Part::Two).await.unwrap()
            );
        }
        Days::Day02 => println!("Still working on it..."),
        _ => println!("Not implemented"),
    }
}
