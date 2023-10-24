use crate::utils::{find_min_max, InstructionCounter};


pub fn sort(arr: &mut [i32], instruction_counter: InstructionCounter) {


    let mut aux_arr = vec![0i32; arr.len()];
    let (min_val, max_val) = find_min_max(&arr);
    let mut counting_arr: Vec<usize> = vec![0usize; (max_val - min_val + 1) as usize];

    let keys: Vec<usize> = arr.iter().map(|&x| ((x - min_val) as usize)).collect();
    
    counting_key_sort(arr, &mut aux_arr, &mut counting_arr, &keys, true);  // Sort based on the last digit
}

pub fn counting_key_sort(arr: &mut [i32], aux_arr: &mut [i32], counting_arr: &mut [usize], keys: &[usize], copy_aux_to_arr: bool) {
    // Count occurrences of each key
    for &k in keys {
        counting_arr[k] += 1;
    }

    // Accumulate counts
    for i in 1..counting_arr.len() {
        counting_arr[i] += counting_arr[i - 1];
    }

    for (num, k) in arr.iter().rev().zip(keys.iter().rev()) {
        aux_arr[counting_arr[*k] - 1] = *num;
        counting_arr[*k] -= 1;
    }

    if copy_aux_to_arr {
        arr.clone_from_slice(&aux_arr);
    }
}
