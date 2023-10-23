
pub fn sort(arr: &mut [i32]) {
    let len = arr.len();
    if len == 0 { return; }
    let p = partition(arr, 0, len - 1);
    recursive_quicksort(arr, 0, p);
    recursive_quicksort(arr, p + 1, len - 1)
}


fn recursive_quicksort(arr: &mut [i32], start_idx: usize, end_idx: usize) {
    if end_idx <= start_idx {
        return;
    }
    let p = partition(arr, start_idx, end_idx);
    if p > 0 {  // Guard against underflow
        recursive_quicksort(arr, start_idx, p - 1);
    }
    recursive_quicksort(arr, p + 1, end_idx);
}

fn partition(arr: &mut [i32], start_idx: usize, end_idx: usize) -> usize {
    let x = arr[end_idx];
    let mut i = start_idx;

    for j in start_idx..end_idx {
        if arr[j] <= x {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, end_idx);
    i
}






