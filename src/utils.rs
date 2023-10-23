pub fn find_min_max(arr: &[i32]) -> (i32, i32) {
    let mut min_val = arr[0];
    let mut max_val = min_val;

    for &num in arr.iter() {
        if num < min_val {
            min_val = num;
        } else if num > max_val {
            max_val = num;
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

pub struct InstructionCounter {
    comparison_count: u128,
    array_access_count: u128,
    div_count: u128,
    bitwise_count: u128
}


pub fn linspace(start: i32, end: i32, num_points: usize) -> Vec<i32> {
    if num_points <= 1 {
        return vec![start];
    }

    let step = (end - start) as f64 / (num_points - 1) as f64;
    (0..num_points).map(|i| (start as f64 + step * i as f64).round() as i32).collect()
}