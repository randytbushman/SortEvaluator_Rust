use std::time::Instant;
use crate::SortingBuffers;

pub fn test_quicksort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
    // Begin timer just before algorithm invocation
    let start = Instant::now();
    quicksort(arr);

    // Return the elapsed time
    start.elapsed().as_micros()
}

/// Sorts an array in-place using quicksort
/// * arr - the array to sort
fn quicksort(arr: &mut [i64]) {
    if arr.len() > 1 {
        let pivot_index = quicksort_partition(arr);
        quicksort(&mut arr[0..pivot_index]);
        quicksort(&mut arr[pivot_index + 1..]);
    }
}

fn quicksort_partition(arr: &mut [i64]) -> usize{
    let mut pivot_index = 0;
    let pivot_value = arr[0];

    for i in 1..arr.len() {
        if arr[i] < pivot_value {
            pivot_index += 1;
            arr.swap(i, pivot_index);
        }
    }

    arr.swap(0, pivot_index);
    pivot_index
}