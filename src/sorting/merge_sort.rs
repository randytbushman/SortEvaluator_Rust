use std::time::Instant;
use crate::SortingBuffers;


pub fn test_merge_sort(arr: &mut [i64], sorting_buffers: &mut SortingBuffers, range: usize) -> u128 {
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

fn merge_sort(arr: &mut [i64], aux_arr_buffer: &mut [i64]) {
    // Ensure buffer is correct size
    assert_eq!(
        arr.len(),
        aux_arr_buffer.len(),
        "buffer length mismatch arr = {}, aux_arr = {})",
        arr.len(),
        aux_arr_buffer.len()
    );
    merge_sort_inner(arr, aux_arr_buffer);
}

fn merge_sort_inner(arr: &mut [i64], aux_arr_buffer: &mut [i64]) {
    if arr.len() > 1 {
        let mid = arr.len() >> 1;
        merge_sort(&mut arr[0.. mid], &mut aux_arr_buffer[0.. mid]);
        merge_sort(&mut arr[mid..], &mut aux_arr_buffer[mid..]);
        merge(arr, aux_arr_buffer);
    }
}

fn merge(arr: &mut [i64], aux_arr_buffer: &mut [i64]) {
    let mid = arr.len() >> 1;
    let left = &arr[0..mid];
    let right = &arr[mid..];

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            aux_arr_buffer[k] = left[i];
            i += 1;
        } else {
            aux_arr_buffer[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Copy the remaining left or right values to the end of the aux_arr
    if i < left.len() {
        aux_arr_buffer[k..].copy_from_slice(&left[i..]);
    } else {
        aux_arr_buffer[k..].copy_from_slice(&right[j..]);
    }

    // Copy the aux_arr to arr
    arr.copy_from_slice(&aux_arr_buffer);
}