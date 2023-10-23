use crate::utils::find_min_max;
use crate::algorithms::countingsort::counting_key_sort;


pub fn sort(arr: &mut [i32], radix: u32) {
    let mut aux_arr = vec![0i32; arr.len()];
    let (min_val, mut max_val) = find_min_max(&arr);

    let mut counting_arr: Vec<usize> = vec![0usize; radix as usize];

    max_val -= min_val;
    let mut exp: i64 = 1;
    let mut is_next_radix = max_val > 0;

    let mut keys: Vec<usize> = arr.iter().map(|&k| ((k - min_val) % radix as i32) as usize).collect();

    // Create pointers for arr and aux_arr to swap during iteration. Prevents excessive arr copies
    let arr_ptr = arr.as_ptr();
    let mut temp_a: &mut [i32] = arr;
    let mut temp_b: &mut [i32] = &mut aux_arr;

    let radix = radix as i64;
    let min_val = min_val as i64;
    let max_val = max_val as i64;

    while is_next_radix {
        exp *= radix;
        is_next_radix = max_val / exp > 0;

        if !is_next_radix {
            counting_key_sort(temp_a, temp_b, &mut counting_arr, &keys, temp_a.as_ptr() == arr_ptr);
        } else {
            counting_key_sort(temp_a, temp_b, &mut counting_arr, &keys, false);
            counting_arr.fill(0);
            temp_b.iter().enumerate().for_each(|(i, &v)| keys[i] = (((v as i64 - min_val) / exp) % radix) as usize);
            std::mem::swap(&mut temp_a, &mut temp_b);
        }
    }
}
