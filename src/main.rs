mod utils;
mod algorithms;

extern crate rand;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use clap::Parser;
use crate::utils::{InstructionCounter, is_sorted, linspace};

/// Program to compare performance of sorting algorithms

#[derive(Parser)]
struct Args {
    /// Number of sorting trials to execute
    #[arg(short = 't', default_value_t = 1)]
    trials: u32,

    /// The size of the smallest array to sort
    #[arg(short = 'i', default_value_t = 10)]
    initial_length: usize,

    /// The size of the largest array to sort
    #[arg(short = 'f', default_value_t = 100)]
    final_length: usize,

    /// The length in which an array increments after each trial period
    #[arg(short = 'l', default_value_t = 10)]
    length_increment: usize,

    /// The minimum possible array value to sort
    #[arg(short = 'm', default_value_t = 0)]
    min_value: i32,

    /// The maximum possible array value to sort
    #[arg(short = 'M', default_value_t = 100)]
    max_value: i32,

    /// The number system radix to use in Radix Sort / the divisor it use in QR Sort
    #[arg(short = 'r', long, default_value_t = 16)]
    divisor_radix: usize,

    /// The maximum number of threads used
    #[arg(short = 'T', default_value_t = 5)]
    threads: u32,

    /// The output filepath location. Outputs are csv-formatted and should have a .csv extension
    #[arg(short = 'o', default_value = "")]
    output_filepath: String
}



fn main() {
    let args = Args::parse();
    let min_value: i32 = args.min_value;
    let max_value: i32 = args.max_value;
    let trials: u32 = args.trials;
    let divisor_radix = args.divisor_radix;

    let algorithms: [Box<dyn Fn(&mut [i32], InstructionCounter)>; 8] = [
        Box::new(algorithms::quicksort::sort),
        Box::new(algorithms::mergesort::sort),
        Box::new(algorithms::countingsort::sort),
        Box::new(|arr, ic| algorithms::radixsort::sort(arr, if divisor_radix > 0 { divisor_radix } else { arr.len() } as u32, ic)),
        Box::new(|arr, ic| algorithms::qr_sort::sort(arr, if divisor_radix > 0 {divisor_radix} else {arr.len()}, false, false, ic)),
        Box::new(|arr, ic| algorithms::qr_sort::sort(arr, if divisor_radix > 0 {divisor_radix} else {arr.len()}, true, false, ic)),
        Box::new(|arr, ic| algorithms::qr_sort::sort(arr, if divisor_radix > 0 {divisor_radix} else {arr.len()}, false, true, ic)),
        Box::new(|arr, ic| algorithms::qr_sort::sort(arr, if divisor_radix > 0 {divisor_radix} else {arr.len()}, true, true, ic)),
    ];

    let mut experienced_error = vec![false; algorithms.len()];
    let mut alg_instruction_counter;


    for arr_len in (args.initial_length..args.final_length).step_by(args.length_increment) {
        let mut arr = linspace(min_value, max_value, arr_len);

        alg_instruction_counter = InstructionCounter {
            comparison_count: 0,
            array_access_count: 0,
            mod_count: 0,
            div_count: 0,
            bitwise_count: 0,
            algorithm_error: false
        };

        for _ in 0..trials {
            arr.shuffle(&mut thread_rng());

            for (i, alg) in algorithms.iter().enumerate() {
                alg(&mut *arr, alg_instruction_counter);
                if alg_instruction_counter.algorithm_error {
                    experienced_error[i] = true;
                    continue;
                }

                if !is_sorted(&arr) {
                    println!("NOPE")
                }
            }

        }

        // Handle processing here


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
