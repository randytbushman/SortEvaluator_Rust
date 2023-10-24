use crate::utils::{find_min_max, InstructionCounter};


pub fn sort(arr: &mut [i32], mut instruction_counter: InstructionCounter) {
    let mut aux_arr = vec![0i32; arr.len()];
    let (min_val, max_val) = find_min_max(&arr, instruction_counter);
    let mut counting_arr: Vec<usize> = vec![0usize; (max_val - min_val + 1) as usize];

    instruction_counter.array_access_count += 2 * arr.len() as u128;
    instruction_counter.comparison_count += arr.len() as u128 + 1;
    let keys: Vec<usize> = arr.iter().map(|&x| ((x - min_val) as usize)).collect();
    
    counting_key_sort(arr, &mut aux_arr, &mut counting_arr, &keys, true, instruction_counter);  // Sort based on the last digit
}

pub fn counting_key_sort(arr: &mut [i32], aux_arr: &mut [i32], counting_arr: &mut [usize], keys: &[usize], copy_aux_to_arr: bool, mut instruction_counter: InstructionCounter) {
    // Count occurrences of each key
    instruction_counter.comparison_count += 1;
    for &k in keys {
        instruction_counter.comparison_count += 1;
        instruction_counter.array_access_count += 1;
        counting_arr[k] += 1;
    }

    // Accumulate counts
    instruction_counter.comparison_count += 1;
    for i in 1..counting_arr.len() {
        instruction_counter.comparison_count += 1;
        instruction_counter.array_access_count += 2;
        counting_arr[i] += counting_arr[i - 1];
    }


    instruction_counter.comparison_count += 1;
    for i in (0..arr.len()).rev() {
        instruction_counter.comparison_count += 1;
        instruction_counter.array_access_count += 5;
        let num = &arr[i];
        let k = &keys[i];
        aux_arr[counting_arr[*k] - 1] = *num;
        counting_arr[*k] -= 1;
    }

    if copy_aux_to_arr {
        arr.clone_from_slice(&aux_arr);
    }
}
