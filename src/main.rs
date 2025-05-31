extern crate rand;
mod utils;
mod sorting;
use rand::seq::SliceRandom;
use clap::Parser;
use rand::rng;
use crate::sorting::{counting_sort, merge_sort, qr_sort, quicksort, radix_sort};
use crate::utils::{is_sorted, linspace};
use std::time::Instant;

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
    #[arg(short = 't', default_value_t = 5)]
    threads: u32,
}



fn main() {
    let args = Args::parse();

    let trials = args.trials;
    let start_length = args.start_length;
    let end_length = args.end_length;
    let length_increment = args.length_increment;
    
    let min_value = args.min_value;
    let max_value = args.max_value;
    let threads = args.threads;

    // A list of algorithms to evaluate
    let algorithms = [
        |arr| {merge_sort(arr)},
        |arr| {quicksort(arr)},
        |arr| {counting_sort(arr, None)},
        |arr| {radix_sort(arr, None)},
        |arr| {qr_sort(arr, None)}
    ];

    // An array that tracks each algorithm's execution time
    let mut algorithm_times: Vec<u128> = vec![0; algorithms.len()];

    for arr_len in (start_length..end_length).step_by(length_increment) {

        // Initialize a linearly spaced array
        let mut arr = linspace(min_value, max_value, arr_len);
        let mut arr_copy = vec![0; arr.len()];

        for _ in 0..trials {
            arr.shuffle(&mut rng());

            for (i, sort_fn) in algorithms.iter().enumerate() {
                arr_copy.clone_from(&arr);

                let start = Instant::now();
                sort_fn(arr_copy);
                // Execute the sorting algorithm 

                algorithm_times[i] += start.elapsed().as_millis();
            }
        }
    }



    /*
    for(int arr_length = initial_length; arr_length <= max_length; arr_length += length_increment) {
        lin_space(arr, arr_length, min_number, max_number);     // Populate arr with linearly spaced values between min_number and max_number

        // Each algorithm sorts the same array sequence on each trial; after each trial, the array is shuffled
        for (i = 0; i < num_trials; ++i) {
            shuffle(arr, arr_length);   // Array is shuffled for each trial
            for (j = 0; j < algorithm_count; ++j)
                algorithm_times[j] += (*sorting_testers[j])(arr, copy_arr, arr_length);
        }

        // Print the average time for each algorithm trial
        printf("%d", arr_length);
        for (i = 0; i < algorithm_count; ++i) {
            printf(", %f", 1000 * algorithm_times[i] / num_trials);
            algorithm_times[i] = 0.0;   // Reset algorithm time after print
        }
        printf("\n");
    }
     */




}
