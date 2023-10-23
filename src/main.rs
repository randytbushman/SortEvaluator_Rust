mod utils;
mod algorithms;

extern crate rand;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

//use clap::Parser;

/// Program to compare performance of sorting algorithms
/*
#[derive(Parser)]
struct Args {
    /// Number of sorting trials to execute
    #[arg(short, long, default_value_t = 1)]
    trials: u32,

    /// The size of the smallest array to sort.
    #[arg(short, long)]
    initial_length: u32,

    /// The size of the largest array to sort.
    #[arg(short, long)]
    final_length: u32,

    /// The length in which an array increments after each trial period.
    #[arg(short, long)]
    length_increment: u32,

    /// The minimum possible array value to sort.
    #[arg(short, long)]
    min_value: i64,

    /// The maximum possible array value to sort.
    #[arg(short, long)]
    max_value: i64,

    /// The output filepath location. Outputs are csv-formatted and should have a .csv extension.
    #[arg(short, long)]
    output_filepath: String
}
*/

fn main() {
    let mut numbers: Vec<i32> = utils::linspace(-120, 20, 20);

    numbers.shuffle(&mut thread_rng());

    for value in &numbers {
        print!("{}, ", value);
    }
    println!();
    algorithms::qr_sort::sort(&mut numbers, 16, false,  true);

    for value in &numbers {
        print!("{}, ", value);
    }
    println!();


    let is_sorted = utils::is_sorted(&numbers);
    println!("Sorted? -> {}", is_sorted)

}
