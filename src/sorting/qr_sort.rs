use std::time::Instant;
use crate::sorting::counting_sort::counting_sort;
use crate::SortingBuffers;
use crate::utils::find_min_max;


pub fn test_qr_sort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
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

/// Sorts an i64 array using divisor = sqrt(n) (optimal) QR Sort
fn qr_sort(arr: &mut [i64], aux_arr_buffer: &mut[i64], keys_buffer: &mut[i64], counting_arr_buffer: &mut[usize]) {
    // An empty array is trivially sorted
    if arr.len() == 0 { return }

    // Ensures that the keys and auxiliary buffers are the correct size
    assert!(
        arr.len() == aux_arr_buffer.len() && arr.len() == keys_buffer.len(),
        "buffer length mismatch arr = {}  aux = {}  keys = {}",
        arr.len(),
        aux_arr_buffer.len(),
        keys_buffer.len(),
    );

    // find min, max, then range
    let (min, max) = find_min_max(&arr);
    let range = max - min;

    // Implies all the elements in arr have equal keys
    if range == 0 { return }

    // Set the divisor equal to the square root of the range
    let divisor = range.isqrt() + 1;

    // First compute, then sort by the remainder keys 
    for (i, key) in keys_buffer.iter_mut().enumerate() {
        *key = (arr[i] - min) % divisor;
    }
    counting_sort(arr, Some(keys_buffer), aux_arr_buffer, counting_arr_buffer);

    // Reset the buffer for the next sorting pass
    counting_arr_buffer.fill(0);

    // Compute then sort by the quotient keys
    for (i, key) in keys_buffer.iter_mut().enumerate() {
        *key = (arr[i] - min) / divisor;
    }
    counting_sort(arr, Some(keys_buffer), aux_arr_buffer, counting_arr_buffer);
}
