use std::time::Instant;
use crate::sorting::counting_sort::counting_sort;
use crate::SortingBuffers;
use crate::utils::find_min_max;


pub fn test_radix_sort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
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

/// Sorts an i64 array using base-n Radix Sort
fn radix_sort(arr: &mut [i64], aux_arr_buffer: &mut[i64], keys_buffer: &mut[i64], counting_arr_buffer: &mut[usize]) {
    // An empty array is trivially sorted
    if arr.len() == 0 { return }

    // find min, max, then range
    let (min, max) = find_min_max(&arr);
    let range = max - min;

    // If the given radix value is None, default to arr.len()
    let radix = arr.len() as i64;

    // Ensures that the keys and auxiliary buffers are the correct size
    assert!(
        arr.len() == aux_arr_buffer.len() && arr.len() == keys_buffer.len(),
        "buffer length mismatch arr = {}  aux = {}  keys = {}",
        arr.len(),
        aux_arr_buffer.len(),
        keys_buffer.len(),
    );

    let mut exp = 1;
    while exp <= range {
        // Compute the digit keys to sort by
        for (value, key) in arr.iter().zip(keys_buffer.iter_mut()) {
            *key = (value - min) / exp % radix;
        }

        // Perform counting sort on the digits
        counting_sort(arr, Some(&keys_buffer), aux_arr_buffer, counting_arr_buffer);
        exp *= radix;

        // Reset the buffer if another pass is required
        if exp <= range {
            counting_arr_buffer.fill(0);
        }
    }
}