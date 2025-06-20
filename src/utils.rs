pub fn is_sorted(arr: &[i64]) -> bool {
    for window in arr.windows(2) {
        if window[0] > window[1] {
            return false;
        }
    }
    true
}

pub fn linspace(start: i64, end: i64, arr_length: usize) -> Vec<i64> {
    if arr_length <= 1 {
        return vec![start];
    }

    let step = (end - start) as f64 / (arr_length - 1) as f64;
    (0..arr_length).map(|i| (start as f64 + step * i as f64).round() as i64).collect()
}
