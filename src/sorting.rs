pub fn merge_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let mid = arr.len() >> 1;
        merge_sort(&mut arr[0.. mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr);
    }
}

pub fn merge(arr: &mut [i32]) {
    let mid = arr.len() >> 1;
    let left = &arr[0..mid];
    let right = &arr[mid..];

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut aux_arr: Vec<i32> = vec![0; arr.len()];
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            aux_arr[k] = left[i];
            i += 1;
        } else {
            aux_arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    
    // Copy the remaining left or right values to the end of the aux_arr
    if i < left.len() {
        aux_arr[k..].copy_from_slice(&left[i..]);
    } else {
        aux_arr[k..].copy_from_slice(&right[j..]);
    }
    
    // Copy the aux_arr to arr
    arr.copy_from_slice(&aux_arr);
}

/// Sorts an array in-place using quicksort
/// * arr - the array to sort
pub fn quicksort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let pivot_index = quicksort_partition(arr);
        quicksort(&mut arr[0..pivot_index]);
        quicksort(&mut arr[pivot_index + 1..]);
    }
}

fn quicksort_partition(arr: &mut [i32]) -> usize{
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

pub fn counting_sort(arr: &mut[i32], keys: Option<&[i32]>) {
    // An empty array is trivially sorted
    if arr.len() == 0 { return }
    
    // If no keys are specified, let the values in arr be their own keys
    let keys = keys.unwrap_or(arr);

    // find min, max, then range
    let (min, max) = find_min_max(&keys);

    // Implies all the elements in keys have equal keys
    if min == max { return }

    // Create an uninitialized i32 auxiliary array
    let mut aux_arr: Vec<i32> = vec![0; arr.len()];
    let mut counting_arr = vec![0; (max - min + 1) as usize];

    // Count the number of occurrences of k in keys
    for &k in keys {
        counting_arr[(k - min) as usize] += 1;
    }

    // Perform cumulative sum
    for i in 1..counting_arr.len() {
        counting_arr[i] += counting_arr[i-1];
    }
    
    // Place elements into the auxiliary array based on their counts
    for i in (0..arr.len()).rev() {
        let new_index = (keys[i] - min) as usize;
        counting_arr[new_index] -= 1;
        aux_arr[counting_arr[new_index]] = arr[i];
    }
        
    // Copy the values back from aux_arr to arr
    arr.copy_from_slice(&aux_arr);
}

pub fn qr_sort(arr: &mut [i32], divisor: Option<i32>) {
    // An empty array is trivially sorted
    if arr.len() == 0 { return }

    // validate custom radix before unwrap
    if let Some(d) = divisor { assert!(d > 0, "divisor must exceed 0") }

    // find min, max, then range
    let (min, max) = find_min_max(&arr);
    let range = max - min;

    // Implies all the elements in arr have equal keys
    if range == 0 { return }

    // Let default d value be sqrt(range(arr))
    let d = divisor.unwrap_or(range.isqrt());

    // Sort by remainders
    let remainders: Vec<i32> = arr.iter().map(|v| (v - min) % d).collect();
    counting_sort(arr, Some(&remainders));

    // Sort by quotients
    let quotients: Vec<i32> = arr.iter().map(|v| (v - min) / d).collect();
    counting_sort(arr, Some(&quotients));
}

pub fn radix_sort(arr: &mut [i32], radix: Option<i32>) {
    // An empty array is trivially sorted
    if arr.len() == 0 { return }

    // validate custom radix before unwrap
    if let Some(r) = radix { assert!(r > 1, "radix must exceed 1"); }

    // find min, max, then range
    let (min, max) = find_min_max(&arr);
    let range = max - min;

    // Implies all the elements in arr have equal keys
    let radix = radix.unwrap_or(arr.len() as i32);
    
    let mut exp = 1;
    while range / exp > 0 {
        // Compute the next set of digits and use them to sort arr
        let digits = arr.iter().map(|v| (v - min) % exp).collect::<Vec<i32>>();
        counting_sort(arr, Some(&digits));
        exp *= radix;
    }
}

fn find_min_max(arr: &[i32]) -> (i32, i32){
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