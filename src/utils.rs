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

pub fn find_min_max(arr: &[i64]) -> (i64, i64){
    assert!(!arr.is_empty(), "slice must contain at least one element");

    let mut min = arr[0];
    let mut max = arr[0];

    for &v in &arr[1..] {
        if v < min {
            min = v;
        } else if v > max {
            max = v;
        }
    }
    (min, max)
}