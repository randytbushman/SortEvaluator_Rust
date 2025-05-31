pub fn is_sorted(arr: &Vec<i32>) -> bool {
    for window in arr.windows(2) {
        if window[0] > window[1] {
            return false;
        }
    }
    true
}

pub fn linspace(start: i32, end: i32, num_points: usize) -> Vec<i32> {
    if num_points <= 1 {
        return vec![start];
    }

    let step = (end - start) as f64 / (num_points - 1) as f64;
    (0..num_points).map(|i| (start as f64 + step * i as f64).round() as i32).collect()
}
