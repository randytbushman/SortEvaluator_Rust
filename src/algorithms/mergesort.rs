

pub fn sort(arr: &mut [i32]) {
    let mid = arr.len() >> 1;
    let mut aux_arr: Vec<i32> = vec![0i32; arr.len()];
    merge_sort_recursive(arr, &mut aux_arr, 0, mid);
    merge_sort_recursive(arr, &mut aux_arr, mid + 1, arr.len() - 1);
    merge(arr, &mut aux_arr, 0, mid, arr.len() - 1);
}

fn merge_sort_recursive(arr: &mut [i32], aux_arr: &mut [i32], start_idx: usize, end_idx: usize) {
    let mid: usize = (start_idx + end_idx) >> 1;
    if start_idx >= end_idx {
        return;
    }
    merge_sort_recursive(arr, aux_arr, start_idx, mid);
    merge_sort_recursive(arr, aux_arr, mid + 1, end_idx);
    merge(arr, aux_arr, start_idx, mid, end_idx);
}

fn merge(arr: &mut [i32], aux_arr: &mut [i32], start_idx: usize, mid_idx: usize, end_idx: usize) {
    let mut i = start_idx;
    let mut j = mid_idx + 1;
    let mut k = 0;

    // Begin merge and store results in aux_arr
    while i <= mid_idx && j <= end_idx {
        if arr[i] < arr[j] {
            aux_arr[k] = arr[i];
            i += 1;
        } else {
            aux_arr[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    // Fill remaining items from the left half into aux_arr
    while i <= mid_idx {
        aux_arr[k] = arr[i];
        i += 1;
        k += 1;
    }

    // Fill remaining items from the right half into aux_arr
    while j <= end_idx {
        aux_arr[k] = arr[j];
        j += 1;
        k += 1;
    }

    // Copy the merged values from aux_arr back into arr
    for x in (start_idx..=end_idx).rev() {
        arr[x] = aux_arr[k - 1];
        k -= 1;
    }
}
