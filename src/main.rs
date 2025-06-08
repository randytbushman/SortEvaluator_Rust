extern crate rand;
mod sorting;
mod utils;

use crate::sorting::{counting_sort, merge_sort, qr_sort, quicksort, radix_sort};
use crate::utils::{linspace};
use std::fmt::Write;    
use itertools::Itertools;
use std::fs::File;
use std::io::Write as IOWrite;
use clap::Parser;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{SeedableRng};
use std::time::Instant;

/// Program to compare performance of sorting algorithms

#[derive(Parser)]
#[command(name = "sort_eval", about = "")]
struct Args {
    /// Number of sorting trials to execute
    #[arg(short = 't', default_value_t = 1)]
    trials: usize,

    /// The size of the smallest array to sort
    #[arg(short = 's', default_value_t = 10)]
    start_length: usize,

    /// The size of the largest array to sort
    #[arg(short = 'e', default_value_t = 100)]
    end_length: usize,

    /// The length in which an array increments after each trial period
    #[arg(short = 'i', default_value_t = 10)]
    length_increment: usize,

    /// The minimum possible array value to sort
    #[arg(short = 'm', default_value_t = 0)]
    min_value: i32,

    ///
    #[arg(short = 'M', long, value_delimiter = ',', value_parser = clap::value_parser!(i32), default_value = "100")]
    max_values: Vec<i32>,
    
    // The maximum possible array value to sort
    //#[arg(short = 'M', default_value_t = 100)]
    //max_value: i32,

    // The value in which the array range increments after each experiment
    //#[arg(short = 'v', default_value_t = 10)]
    //value_increment: usize,

    /// The maximum number of threads used
    #[arg(short = 'w', default_value_t = 5)]
    threads: usize,

    #[arg(short = 'o', default_value_t = String::from("./results"))]
    output_dir: String,
}

fn main() {
    let args = Args::parse();

    // Select seed to get reproducible results
    let seed: u64 = 42; // choose your seed
    let mut rng = StdRng::seed_from_u64(seed);

    // Parse command line arguments
    let trials = args.trials;
    let start_length = args.start_length;
    let end_length = args.end_length;
    let length_increment = args.length_increment;
    let min_value = args.min_value;
    let max_values = args.max_values;
    let output_dir = args.output_dir;

    // A list of algorithms to evaluate
    let algorithms: [fn(&mut [i32]); 5] = [
        merge_sort,
        quicksort,
        |arr| counting_sort(arr, None),
        |arr| radix_sort(arr, None),
        |arr| qr_sort(arr, None),
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

    for max_value in max_values.iter() {
        println!("Begin experiment with range {min_value}-{max_value}");

        // A string that captures all the experiment information
        let mut experiment_text: String = std::iter::once("Length")
            .chain(algorithm_names.iter().copied())
            .join(",")
            + "\n";
        
        // From start length to end length perform trials to evaluate algorithm performance
        for arr_len in (start_length..=end_length).step_by(length_increment) {
            
            println!("Begin trials with length {arr_len}");
            // Initialize a linearly spaced array
            let mut base_arr = linspace(min_value, *max_value, arr_len);
            let mut arr_copy = vec![0; arr_len];

            // Execute algorithm trials
            for _ in 0..trials {
                // Shuffle the base array for each new trial
                base_arr.shuffle(&mut rng);

                // Iterate though each algorithm callable and capture execution time
                for (i, sort_fn) in algorithms.iter().enumerate() {
                    // Copy the base array so each algorithm sorts the same sequences
                    arr_copy.clone_from(&base_arr);

                    // Execute the sorting algorithm and track the time it takes
                    let start = Instant::now();
                    sort_fn(&mut arr_copy);

                    //if !is_sorted(&arr_copy) {
                    //    eprintln!("Failed to sort array using {}.", algorithm_names[i]);
                    // }

                    algorithm_times[i] += start.elapsed().as_micros();
                }
            }
            
            // Write the array length to the experiments string
            write!(&mut experiment_text, "{}", arr_len).expect("Could not write array length to experiment string");
            
            // Compute the averages of each algorithm time 
            let averages = algorithm_times
                .iter_mut()
                .map(|time| {
                    let average = *time / (trials as u128);
                    *time = 0;
                    format!(",{}", average)
                })
                .join("");

            // Write the averages to the experiment_text
            write!(experiment_text, "{}\n", averages).expect("Could not averages write to string");
        }
        
        // After completing an experiment by length, write the results to a file
        let filename = format!("{output_dir}/{min_value}min_value-{max_value}max_value.csv");
        write_string_to_file(&filename, &experiment_text);
        
        println!("Done with experiment {filename}");
        
        // Clear for next experiment 
        experiment_text.clear()
    }
}

fn write_string_to_file(filename: &str, data: &String) {
    let mut file = File::create(filename).expect(&format!("Could not open file: {filename}"));
    file.write_all(data.as_bytes()).unwrap();
}
