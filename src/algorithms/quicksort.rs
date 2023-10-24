use crate::utils::InstructionCounter;

pub fn sort(arr: &mut [i32], mut instruction_counter: InstructionCounter) {
    let len = arr.len();

    if len == 0 { return; }
    instruction_counter.comparison_count += 1;

    let p = partition(arr, 0, len - 1, instruction_counter);
    recursive_quicksort(arr, 0, p, instruction_counter);
    recursive_quicksort(arr, p + 1, len - 1, instruction_counter)
}


fn recursive_quicksort(arr: &mut [i32], start_idx: usize, end_idx: usize, mut instruction_counter: InstructionCounter) {
    instruction_counter.comparison_count += 1;
    if end_idx <= start_idx {
        return;
    }
    let p = partition(arr, start_idx, end_idx, instruction_counter);

    instruction_counter.comparison_count += 1;
    if p > 0 {  // Guard against underflow
        recursive_quicksort(arr, start_idx, p - 1, instruction_counter);
    }

    recursive_quicksort(arr, p + 1, end_idx, instruction_counter);
}

fn partition(arr: &mut [i32], start_idx: usize, end_idx: usize, mut instruction_counter: InstructionCounter) -> usize {
    instruction_counter.array_access_count += 1;
    let x = arr[end_idx];
    let mut i = start_idx;

    for j in start_idx..end_idx {
        instruction_counter.array_access_count += 1;
        instruction_counter.comparison_count += 1;
        if arr[j] <= x {
            instruction_counter.array_access_count += 4;  // Number of array accesses in swap operation
            arr.swap(i, j);
            i += 1;
        }
    }

    instruction_counter.array_access_count += 4;
    arr.swap(i, end_idx);
    i
}






