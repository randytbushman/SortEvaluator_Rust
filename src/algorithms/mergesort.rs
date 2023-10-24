use crate::utils::InstructionCounter;

pub fn sort(arr: &mut [i32], mut ic: InstructionCounter) {
    ic.bitwise_count += 1;
    let mid = arr.len() >> 1;
    let mut aux_arr: Vec<i32> = vec![0i32; arr.len()];
    merge_sort_recursive(arr, &mut aux_arr, 0, mid, ic);
    merge_sort_recursive(arr, &mut aux_arr, mid + 1, arr.len() - 1, ic);
    merge(arr, &mut aux_arr, 0, mid, arr.len() - 1, ic);
}

fn merge_sort_recursive(arr: &mut [i32], aux_arr: &mut [i32], start_idx: usize, end_idx: usize, mut ic: InstructionCounter) {
    ic.bitwise_count += 1;
    let mid: usize = (start_idx + end_idx) >> 1;

    ic.comparison_count += 1;
    if start_idx >= end_idx {
        return;
    }
    merge_sort_recursive(arr, aux_arr, start_idx, mid, ic);
    merge_sort_recursive(arr, aux_arr, mid + 1, end_idx, ic);
    merge(arr, aux_arr, start_idx, mid, end_idx, ic);
}

fn merge(arr: &mut [i32], aux_arr: &mut [i32], start_idx: usize, mid_idx: usize, end_idx: usize, mut ic: InstructionCounter) {
    let mut i = start_idx;
    let mut j = mid_idx + 1;
    let mut k = 0;

    // Begin merge and store results in aux_arr
    while { ic.comparison_count += 1; i <= mid_idx && { ic.comparison_count += 1; j <= end_idx }}
    {
        ic.comparison_count += 1;
        ic.array_access_count += 2;
        if arr[i] < arr[j] {
            ic.array_access_count += 2;
            aux_arr[k] = arr[i];
            i += 1;
        } else {
            ic.array_access_count += 2;
            aux_arr[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    // Fill remaining items from the left half into aux_arr
    while { ic.comparison_count += 1; i <= mid_idx }
    {
        ic.array_access_count += 2;
        aux_arr[k] = arr[i];
        i += 1;
        k += 1;
    }

    // Fill remaining items from the right half into aux_arr
    while { ic.comparison_count += 1; j <= end_idx }
    {
        ic.array_access_count += 2;
        aux_arr[k] = arr[j];
        j += 1;
        k += 1;
    }

    ic.array_access_count += 2 * arr.len() as u128;
    ic.comparison_count += arr.len() as u128 + 1;
    arr.clone_from_slice(&aux_arr);
}
