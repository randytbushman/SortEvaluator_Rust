pub fn find_min_max(arr: &[i32], mut instruction_counter: InstructionCounter) -> (i32, i32) {
    instruction_counter.array_access_count += 1;
    let mut min_val = arr[0];
    let mut max_val = min_val;

    instruction_counter.comparison_count += 1;  // Initial for loop comparison
    for &num in arr.iter() {
        instruction_counter.comparison_count += 1; // For loop comparison

        if num < min_val {
            instruction_counter.comparison_count += 1;
            min_val = num;
        } else if num > max_val {
            instruction_counter.comparison_count += 2;
            max_val = num;
        } else {
            instruction_counter.comparison_count += 2;
        }
    }

    (min_val, max_val)
}

pub fn is_sorted(arr: &Vec<i32>) -> bool {
    for window in arr.windows(2) {
        if window[0] > window[1] {
            return false;
        }
    }
    true
}

#[derive(Clone, Copy)]
pub struct InstructionCounter {
    pub comparison_count: u128,
    pub array_access_count: u128,
    pub mod_count: u128,
    pub div_count: u128,
    pub bitwise_count: u128,
    pub algorithm_error: bool
}


pub fn linspace(start: i32, end: i32, num_points: usize) -> Vec<i32> {
    if num_points <= 1 {
        return vec![start];
    }

    let step = (end - start) as f64 / (num_points - 1) as f64;
    (0..num_points).map(|i| (start as f64 + step * i as f64).round() as i32).collect()
}