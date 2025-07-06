pub fn merge_sort(arr: &mut [i64], aux_arr_buffer: &mut [i64]) {
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

pub fn merge(arr: &mut [i64], aux_arr_buffer: &mut [i64]) {
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

/// Sorts an array in-place using quicksort
/// * arr - the array to sort
pub fn quicksort(arr: &mut [i64]) {
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

/// Sorts an i64 array using divisor = sqrt(n) (optimal) QR Sort
/// 
pub fn qr_sort(arr: &mut [i64], aux_arr_buffer: &mut[i64], keys_buffer: &mut[i64], counting_arr_buffer: &mut[usize]) {
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

/// Sorts an i64 array using base-n Radix Sort
/// 
pub fn radix_sort(arr: &mut [i64], aux_arr_buffer: &mut[i64], keys_buffer: &mut[i64], counting_arr_buffer: &mut[usize]) {
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

fn find_min_max(arr: &[i64]) -> (i64, i64){
    assert!(!arr.is_empty(), "slice must contain at least one element");
    
    let mut min = arr[0];
    let mut max = arr[0];
    
    for &v in &arr[1..] {
        if v < min {
            min = v;
        } else if v > max {
            max = v;
        }
    }
    (min, max)
}