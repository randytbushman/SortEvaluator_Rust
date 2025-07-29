extern crate rand;
mod sorting;
mod utils;
use crate::sorting::{counting_sort, merge_sort, qr_sort, quicksort, radix_sort};
use crate::utils::{is_sorted, linspace};
use itertools::Itertools;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::fmt::Write;
use std::fs::File;
use std::io::Write as IOWrite;
use std::time::Instant;

struct SortingBuffers {
    aux_buffer: Vec<i64>,
    keys_buffer: Vec<i64>,
    counting_buffer: Vec<usize>,
}

impl SortingBuffers {
    fn new_from_experiment(experiment: &SortingExperiment) -> Self {
        let max_range_value = experiment
            .max_values
            .iter()
            .max()
            .expect("Error parsing max values.")
            - experiment.min_value;
        Self {
            aux_buffer: vec![0; experiment.end_length],
            keys_buffer: vec![0; experiment.end_length],
            counting_buffer: vec![0; ((max_range_value + 1) as usize).max(experiment.end_length)],
        }
    }
}

struct SortingExperiment {
    trials: usize,
    start_length: usize,
    end_length: usize,
    length_inc: usize,
    min_value: i64,
    max_values: Vec<i64>,
    output_dir: &'static str,
    algorithm_name_headers: Vec<&'static str>,
    algorithm_functions: Vec<fn (arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128>,
    random_seed_rng: StdRng,
}

impl SortingExperiment {
    fn new_all() -> Self {
        Self {
            trials:         10,
            start_length:   10_000,
            end_length:     1_000_000,
            length_inc:     10_000,
            min_value:      0,
            max_values: (4..8).map(|exp| 10i64.pow(exp)).collect(),
            output_dir: "./results_all",
            algorithm_name_headers: vec!["Quicksort", "Merge Sort", "Counting Sort", "Radix Sort", "QR Sort"],
            algorithm_functions: vec![run_quicksort, run_merge_sort, run_counting_sort, run_radix_sort, run_qr_sort],
            random_seed_rng: StdRng::seed_from_u64(42),
        }
    }

    fn new_rs_qr() -> Self {
        Self {
            trials:         10,
            start_length:   10_000,
            end_length:     1_000_000,
            length_inc:     10_000,
            min_value:      0,
            max_values: (4..=12).map(|exp| 10i64.pow(exp)).collect(),
            output_dir: "./results_radix_qr_dev",
            algorithm_name_headers: vec!["Radix Sort", "QR Sort"],
            algorithm_functions: vec![run_radix_sort, run_qr_sort],
            random_seed_rng: StdRng::seed_from_u64(42),
        }
    }

    fn new_qr_ms_qs_cs() -> Self {
        Self {
            trials:         10,
            start_length:   10_000,
            end_length:     1_000_000,
            length_inc:     10_000,
            min_value:      0,
            max_values: vec![10i64.pow(10)],
            output_dir: "./results_qr_ms_qs_cs",
            algorithm_name_headers: vec!["Quicksort", "Merge Sort", "QR Sort"],
            algorithm_functions: vec![run_quicksort, run_merge_sort, run_qr_sort],
            random_seed_rng: StdRng::seed_from_u64(42),
        }
    }

    fn run_experiment(self: &mut Self) {
        // Tallies the time it takes in Microseconds each algorithm takes
        let mut algorithm_times: Vec<u128> = vec![0; self.algorithm_functions.len()];

        // Initializes mutable buffers that can be used by each sorting algorithm
        let mut sorting_buffers = SortingBuffers::new_from_experiment(&self);

        // Iterate through each of the max values defined
        for max_value in self.max_values.iter() {

            // Compute the range of the array to be sorted 
            let range = (*max_value - self.min_value) as usize;
            println!("Begin experiment with range {}-{max_value}", self.min_value);

            // A string that captures all the experiment headers information
            let mut header_text: String = std::iter::once("Length")
                .chain(self.algorithm_name_headers.iter().copied())
                .join(",")
                + "\n";

            // From start length to end length perform trials to evaluate algorithm performance
            for arr_len in (self.start_length..=self.end_length).step_by(self.length_inc) {
                println!("Begin trials with length {arr_len}");

                // Initialize a linearly spaced array and a copy
                let mut base_arr = linspace(self.min_value, *max_value, arr_len);
                let mut arr_copy = vec![0; arr_len];

                // Each experiment executes `trials` number of trials
                for _ in 0..self.trials {
                    base_arr.shuffle(&mut self.random_seed_rng);

                    // Iterate though each algorithm function, execute it, and tally its runtime
                    for (i, sort_fn) in self.algorithm_functions.iter().enumerate() {
                        // Copy the base array so each algorithm sorts the same sequences
                        arr_copy.clone_from(&base_arr);

                        // Execute the sorting algorithm and track the time (ms) it takes
                        let start = Instant::now();
                        sort_fn(&mut arr_copy, &mut sorting_buffers, range);
                        algorithm_times[i] += start.elapsed().as_micros();

                        //if !is_sorted(&arr_copy) {
                        //    eprintln!("Failed to sort array using {}.", algorithm_names[i]);
                        //}
                    }
                }

                // Write the array length to the experiments string
                write!(&mut header_text, "{arr_len}")
                    .expect("Could not write array length to experiment string");

                // Compute the averages of each algorithm time and reset the array
                let averages = algorithm_times
                    .iter_mut()
                    .map(|time| {
                        let average = *time / (self.trials as u128);
                        *time = 0;
                        format!(",{average}")
                    })
                    .join("");

                // Write the averages to the experiment_text
                write!(header_text, "{averages}\n")
                    .expect("Could not averages write to string");
            }

            // After completing an experiment by length, write the results to a file
            let filename = format!("{}/{}min_value-{max_value}max_value.csv", self.output_dir, self.min_value);
            write_string_to_file(&filename, &header_text);

            println!("Done with experiment {filename}");

            // Clear for next experiment
            header_text.clear()
        }
    }
}

fn write_string_to_file(filename: &str, data: &String) {
    let mut file = File::create(filename)
        .expect(&format!("Could not open file: {filename}"));
    file.write_all(data.as_bytes()).unwrap();
}

fn run_merge_sort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
    // Declare all heap variables before marking start time
    let aux_arr_buffer = &mut sorting_buffers.aux_buffer[..arr.len()];

    // Begin timer just before algorithm invocation
    let time_tracker = Instant::now();
    merge_sort(arr, aux_arr_buffer);
    let elapsed = time_tracker.elapsed().as_micros();

    // Reset the buffer slices
    aux_arr_buffer.fill(0);
    elapsed
}

fn run_quicksort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
    // Begin timer just before algorithm invocation
    let start = Instant::now();
    quicksort(arr);

    // Return the elapsed time
    start.elapsed().as_micros()
}

fn run_counting_sort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
    // Create mutable slices of the buffers
    let aux_arr_buffer = &mut sorting_buffers.aux_buffer[..arr.len()];
    let counting_buffer = &mut sorting_buffers.counting_buffer[..=range];

    // Track the time for Counting Sort to run
    let time_tracker = Instant::now();
    counting_sort(arr, None, aux_arr_buffer, counting_buffer);
    let elapsed = time_tracker.elapsed().as_micros();

    // Reset the buffer slices
    aux_arr_buffer.fill(0);
    counting_buffer.fill(0);

    elapsed
}

fn run_qr_sort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
    // Create mutable slices of the buffers
    let aux_arr_buffer = &mut sorting_buffers.aux_buffer[..arr.len()];
    let key_buffer = &mut sorting_buffers.keys_buffer[..arr.len()];
    let counting_buffer = &mut sorting_buffers.counting_buffer[..=range.isqrt()];

    // Track the time for Counting Sort to run
    let time_tracker = Instant::now();
    qr_sort(arr, aux_arr_buffer, key_buffer, counting_buffer);
    let elapsed = time_tracker.elapsed().as_micros();

    // Reset the buffer slices
    aux_arr_buffer.fill(0);
    key_buffer.fill(0);
    counting_buffer.fill(0);

    elapsed
}

fn run_radix_sort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
    // Create mutable slices of the buffers
    let aux_arr_buffer = &mut sorting_buffers.aux_buffer[..arr.len()];
    let key_buffer = &mut sorting_buffers.keys_buffer[..arr.len()];
    let counting_buffer = &mut sorting_buffers.counting_buffer[..arr.len()];

    // Track the time for Radix Sort to run
    let time_tracker = Instant::now();
    radix_sort(arr, aux_arr_buffer, key_buffer, counting_buffer);
    let elapsed = time_tracker.elapsed().as_micros();

    // Reset the buffer slices
    aux_arr_buffer.fill(0);
    key_buffer.fill(0);
    counting_buffer.fill(0);

    elapsed
}

fn main() {
    SortingExperiment::new_qr_ms_qs_cs().run_experiment();
    //SortingExperiment::new_all_experiment().run_experiment();
}
