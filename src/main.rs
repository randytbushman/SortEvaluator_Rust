extern crate rand;
mod utils;
mod sorting;
use rand::seq::SliceRandom;
use clap::Parser;
use rand::{rng, SeedableRng};
use crate::sorting::{counting_sort, merge_sort, parallel_quicksort, qr_sort, quicksort, radix_sort};
use crate::utils::{is_sorted, linspace};
use std::time::Instant;
use rand::rngs::StdRng;

/// Program to compare performance of sorting algorithms

#[derive(Parser)]
#[command(
    name = "sort_eval",
    about = ""
)]
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

    /// The maximum possible array value to sort
    #[arg(short = 'M', default_value_t = 100)]
    max_value: i32,

    /// The maximum number of threads used
    #[arg(short = 'w', default_value_t = 5)]
    threads: usize,
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
    let max_value = args.max_value;
    let threads = args.threads;

    // A list of algorithms to evaluate
    let algorithms: [fn(&mut [i32]); 5] = [
        merge_sort,
        quicksort,
        |arr| { counting_sort(arr, None) },
        |arr| { radix_sort(arr, None) },
        |arr| { qr_sort(arr, None) }
    ];
    
    let algorithm_names = [
        "Merge Sort",
        "Quick Sort",
        "Counting Sort",
        "Radix Sort",
        "QR Sort",
    ];

    // Print sorting algorithm names
    print!("Length");
    for name in algorithm_names.iter() {
        print!(",{}", name);
    }
    println!();

    // An array that tracks each algorithm's execution time
    let mut algorithm_times: Vec<u128> = vec![0; algorithms.len()];

    // Build a Rayon thread pool with the desired number of threads
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("Failed to create thread pool");

    // From start length to end length perform trials to evaluate algorithm performance
    for arr_len in (start_length..=end_length).step_by(length_increment) {

        // Initialize a linearly spaced array
        let mut base_arr = linspace(min_value, max_value, arr_len);
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
                
                if !is_sorted(&arr_copy) {
                    eprintln!("Failed to sort array using {}.", algorithm_names[i]);
                }
                
                algorithm_times[i] += start.elapsed().as_micros();
            }
        }
        
        // Print algorithm statistics
        print!("{}", arr_len);
        for i in 0..algorithm_times.len() {
            
            // Print average execution time
            print!(",{}", algorithm_times[i] / (trials as u128));

            // Rest algorithm times for next experiments
            algorithm_times[i] = 0;  
        }
        println!();
    }
}
