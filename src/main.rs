extern crate rand;
mod sorting;
mod utils;

use crate::sorting::{counting_sort, merge_sort, qr_sort, quicksort, radix_sort};
use crate::utils::{is_sorted, linspace};
use std::fmt::{format, Write};    
use itertools::Itertools;
use std::fs::File;
use std::io::Write as IOWrite;
use clap::Parser;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{SeedableRng};
use std::time::Instant;

use std::fs::OpenOptions;
use std::path::Path;

/// Program to compare performance of sorting algorithms

#[derive(Parser)]
#[command(name = "sort_eval", about = "")]
struct Args {
    /// Number of sorting trials to execute
    #[arg(short = 't', default_value_t = 3)]
    trials: usize,

    /// The size of the smallest array to sort
    #[arg(short = 's', default_value_t = 10000)]
    start_length: usize,

    /// The size of the largest array to sort
    #[arg(short = 'e', default_value_t = 1000000)]
    end_length: usize,

    /// The length in which an array increments after each trial period
    #[arg(short = 'i', default_value_t = 10000)]
    length_increment: usize,

    /// The minimum possible array value to sort
    #[arg(short = 'm', default_value_t = 0)]
    min_value: i64,

    ///
    #[arg(short = 'M', long, value_delimiter = ',', value_parser = clap::value_parser!(i64), default_value = "1000000000")]
    max_values: Vec<i64>,
    
    /// The maximum number of threads used
    #[arg(short = 'w', default_value_t = 5)]
    threads: usize,

    #[arg(short = 'o', default_value_t = String::from("./results"))]
    output_dir: String,
}

fn main() {
    let args = Args::parse();

    // Check if the output
    let output_dir = args.output_dir;
    assert!(!filepath_allows_write(&output_dir), "Output filepath is invalid or not writable.");

    // Parse command line arguments
    let trial_count = args.trials;
    let start_length = args.start_length;
    let end_length = args.end_length;
    let length_increment = args.length_increment;
    let min_value = args.min_value;
    let max_values = args.max_values;

    // Select seed to get reproducible results
    let seed: u64 = 42; // choose your seed
    let mut rng = StdRng::seed_from_u64(seed);

    // A list of algorithms to evaluate
    let algorithms = [
        run_merge_sort,
        run_quicksort,
        run_counting_sort,
        run_radix_sort,
        run_qr_sort,
    ];

    // The accompanying names of the algorithms defined above
    let algorithm_names = [
        "Merge Sort",
        "Quick Sort",
        "Counting Sort",
        "Radix Sort",
        "QR Sort",
    ];

    // An array that tracks each algorithm's execution time
    let mut algorithm_times: Vec<u128> = vec![0; algorithms.len()];

    // Iterate through the given max values (users can input many max values that make a new experiment)
    for max_value in max_values.iter() {
        let range = (*max_value - min_value) as usize;
        println!("Begin experiment with range {min_value}-{max_value}");
        
        
        // A string that captures all the experiment headers information
        let mut experiment_text: String = std::iter::once("Length")
            .chain(algorithm_names.iter().copied())
            .join(",")
            + "\n";
        
        // From start length to end length perform trials to evaluate algorithm performance
        for arr_len in (start_length..=end_length).step_by(length_increment) {
            println!("Begin trials with length {arr_len}");
           
            // Initialize a linearly spaced array and a copy
            let mut base_arr = linspace(min_value, *max_value, arr_len);
            let mut arr_copy = vec![0; arr_len];

            // Each experiment executes `trial_count` trials
            // Each algorithm sorts the same copy of the base array 
            // The base array is shuffled for every trial
            for _ in 0..trial_count {
                base_arr.shuffle(&mut rng);

                // Iterate though each algorithm, execute it, and capture its runtime
                for (i, sort_fn) in algorithms.iter().enumerate() {
                    
                    // Copy the base array so each algorithm sorts the same sequences
                    arr_copy.clone_from(&base_arr);

                    // Execute the sorting algorithm and track the time (ms) it takes
                    let start = Instant::now();
                    sort_fn(&mut arr_copy, range);
                    algorithm_times[i] += start.elapsed().as_micros();

                    //if !is_sorted(&arr_copy) {
                    //    eprintln!("Failed to sort array using {}.", algorithm_names[i]);
                    //}
                }
            }
            
            // Write the array length to the experiments string
            write!(&mut experiment_text, "{arr_len}").expect("Could not write array length to experiment string");
            
            // Compute the averages of each algorithm time and reset the array
            let averages = algorithm_times
                .iter_mut()
                .map(|time| {
                    let average = *time / (trial_count as u128);
                    *time = 0;
                    format!(",{average}")
                })
                .join("");

            // Write the averages to the experiment_text
            write!(experiment_text, "{averages}\n").expect("Could not averages write to string");
        }
        
        // After completing an experiment by length, write the results to a file
        let filename = format!("{output_dir}/{min_value}min_value-{max_value}max_value.csv");
        write_string_to_file(&filename, &experiment_text);
        
        println!("Done with experiment {filename}");
        
        // Clear for next experiment 
        experiment_text.clear()
    }
}

/// Checks write permission and validity for a file path.
/// :return: true if writable, false otherwise
fn filepath_allows_write(dir_path: &str) -> bool {
    OpenOptions::new().write(true).create(true).open(Path::new(dir_path)).is_ok()
}

fn write_string_to_file(filename: &str, data: &String) {
    let mut file = File::create(filename).expect(&format!("Could not open file: {filename}"));
    file.write_all(data.as_bytes()).unwrap();
}

fn run_merge_sort(arr: &mut [i64], range: usize) -> u128 {
    // Declare all heap variables before marking start time
    let mut aux_arr= vec![0; arr.len()];
    
    // Begin timer just before algorithm invocation
    let start = Instant::now();
    merge_sort(arr, &mut aux_arr);
    
    // Return the elapsed time
    start.elapsed().as_micros()
}

fn run_quicksort(arr: &mut [i64], range: usize) -> u128 {
    // Begin timer just before algorithm invocation
    let start = Instant::now();
    quicksort(arr);

    // Return the elapsed time
    start.elapsed().as_micros()
}

fn run_counting_sort(arr: &mut [i64], range: usize) -> u128 {
    let mut aux_arr_buffer= vec![0; arr.len()];
    let mut counting_arr_buffer = vec![0; range + 1];

    let start = Instant::now();
    counting_sort(arr, None, &mut aux_arr_buffer, &mut counting_arr_buffer);
    start.elapsed().as_micros()
}

fn run_qr_sort(arr: &mut [i64], range: usize) -> u128 {
    let mut keys_buffer = vec![0; arr.len()];
    let mut aux_arr_buffer= vec![0; arr.len()];
    let mut counting_arr_buffer = vec![0; range.isqrt() + 1];

    let start = Instant::now();
    qr_sort(arr, &mut keys_buffer, &mut aux_arr_buffer, &mut counting_arr_buffer);
    start.elapsed().as_micros()
}

fn run_radix_sort(arr: &mut [i64], range: usize) -> u128 {
    let mut keys_buffer = vec![0; arr.len()];
    let mut aux_arr_buffer= vec![0; arr.len()];
    let mut counting_arr_buffer = vec![0; arr.len()];

    let start = Instant::now();
    radix_sort(arr, &mut keys_buffer, &mut aux_arr_buffer, &mut counting_arr_buffer);
    start.elapsed().as_micros()
}
