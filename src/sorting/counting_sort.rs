use std::time::Instant;
use crate::SortingBuffers;
use crate::utils::find_min_max;

pub fn test_counting_sort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
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

/// Sorts an array using Counting Sort with the given keys (by default arr = keys)
///
pub fn counting_sort(arr: &mut[i64], keys: Option<&[i64]>, aux_arr_buffer: &mut[i64], counting_arr_buffer: &mut[usize]) {
    // An empty array is trivially sorted
    if arr.len() == 0 { return }

    // If no keys are specified, let the values in arr be their own keys
    let keys = keys.unwrap_or(arr);

    // Ensures that the keys and auxiliary buffers are the correct size
    assert!(
        arr.len() == aux_arr_buffer.len() && arr.len() == keys.len(),
        "buffer length mismatch arr = {}  aux = {}  keys = {}",
        arr.len(),
        aux_arr_buffer.len(),
        keys.len(),
    );

    // find min, max, then range
    let (min, max) = find_min_max(&keys);

    // Implies all the elements in have equal keys, making arr sorted
    if min == max { return }

    // Ensures the counting_arr buffer is the correct size
    assert!(
        counting_arr_buffer.len() >= (max - min + 1) as usize,
        "counting_arr_buffer length mismatch: range = {}, counting_arr_buffer length = {}",
        max - min + 1,
        counting_arr_buffer.len()
    );

    // Use only the minimum buffer size needed for the counting array
    let counting_arr_buffer = &mut counting_arr_buffer[..(max - min + 1) as usize];

    // Count the number of occurrences of k in keys
    for &k in keys {
        counting_arr_buffer[(k - min) as usize] += 1;
    }

    // Perform cumulative sum
    for i in 1..counting_arr_buffer.len() {
        counting_arr_buffer[i] += counting_arr_buffer[i-1];
    }

    // Place elements into the auxiliary array based on their counts
    for i in (0..arr.len()).rev() {
        let new_index = (keys[i] - min) as usize;
        counting_arr_buffer[new_index] -= 1;
        aux_arr_buffer[counting_arr_buffer[new_index]] = arr[i];
    }

    // Copy the values back from aux_arr to arr
    arr.copy_from_slice(&aux_arr_buffer);
}