use crate::utils::{find_min_max, InstructionCounter};
use crate::algorithms::countingsort::counting_key_sort;


pub fn sort(arr: &mut [i32], radix: u32, mut ic: InstructionCounter) {
    let (min_val, mut max_val) = find_min_max(&arr, ic);


    max_val -= min_val;
    let mut exp: i64 = 1;
    let mut is_next_radix = max_val > 0;

    let mut aux_arr = vec![0i32; arr.len()];
    let mut counting_arr: Vec<usize> = vec![0usize; radix as usize];

    ic.comparison_count += arr.len() as u128 + 1;
    ic.array_access_count += 2 * arr.len() as u128;
    ic.mod_count += arr.len() as u128;
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

        ic.div_count += 1;
        is_next_radix = max_val / exp > 0;

        if !is_next_radix {
            ic.comparison_count += 1;
            counting_key_sort(temp_a, temp_b, &mut counting_arr, &keys, temp_a.as_ptr() == arr_ptr, ic);
        } else {
            counting_key_sort(temp_a, temp_b, &mut counting_arr, &keys, false, ic);

            ic.array_access_count += counting_arr.len() as u128;
            ic.comparison_count += counting_arr.len() as u128 + 1;
            counting_arr.fill(0);

            ic.array_access_count += 2 * temp_a.len() as u128;
            ic.comparison_count += temp_a.len() as u128 + 1;
            ic.mod_count += temp_a.len() as u128;
            ic.div_count += temp_a.len() as u128;
            temp_b.iter().enumerate().for_each(|(i, &v)| keys[i] = (((v as i64 - min_val) / exp) % radix) as usize);

            ic.array_access_count += 4;
            std::mem::swap(&mut temp_a, &mut temp_b);
        }
    }
}
