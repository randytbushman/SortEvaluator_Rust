use std::cmp;
use crate::utils::{find_min_max, InstructionCounter};
use crate::algorithms::countingsort::counting_key_sort;


pub fn sort(arr: &mut [i32], divisor: usize, min_val_zero: bool, use_bitwise: bool, mut ic: InstructionCounter) {
    let (min_val, max_val) = find_min_max(&arr, ic);

    ic.div_count += 1;
    let max_quotient = ((max_val - min_val) as usize / divisor) + 1;

    ic.comparison_count += 1;
    let k = if divisor > max_quotient { divisor + 1 } else { max_quotient };

    let mut counting_arr: Vec<usize> = vec![0usize; k];
    let mut aux_arr = vec![0i32; arr.len()];

    let mod_key_function: Box<dyn Fn(i32) -> usize>;
    let div_key_function: Box<dyn Fn(i32) -> usize>;
    let d = divisor as i32;
    let p;

    if use_bitwise {
        if divisor <= 0 || (divisor & (divisor - 1)) != 0 {  // Cannot use bitwise since divisor is not power of 2
            ic.algorithm_error = true;
            return;
        }
        p = divisor.ilog2() as i32;
        ic.bitwise_count += 2 * arr.len() as u128;  // Count beforehand to distinguish between regular and bitwise operators
        if min_val_zero {
            mod_key_function = Box::new(|x: i32| ((x & (d - 1)) as usize));
            div_key_function = Box::new(|x: i32| ((x >> p) as usize));
        } else {
            mod_key_function = Box::new(|x: i32| (((x - min_val) & (d - 1)) as usize));
            div_key_function = Box::new(|x: i32| (((x - min_val) >> p) as usize));
        }
    } else {
        ic.mod_count += arr.len() as u128;  // Count beforehand to distinguish between regular and bitwise operators
        ic.div_count += arr.len() as u128;
        if min_val_zero {
            mod_key_function = Box::new(|x: i32| ((x % d) as usize));
            div_key_function = Box::new(|x: i32| ((x / d) as usize));
        } else {
            mod_key_function = Box::new(|x: i32| (((x - min_val) % d) as usize));
            div_key_function = Box::new(|x: i32| (((x - min_val) / d) as usize));
        }
    }
    qr_sort(arr, &mut aux_arr, &mut counting_arr, divisor, max_quotient, mod_key_function, div_key_function, ic);
}


fn qr_sort<E, F>(arr: &mut [i32], aux_arr: &mut [i32], counting_arr: &mut [usize], divisor: usize, max_quotient: usize, mod_key_function: E, div_key_function: F, mut ic: InstructionCounter)
where E: Fn(i32) -> usize,
      F: Fn(i32) -> usize,
{
    ic.comparison_count += arr.len() as u128 + 1;
    ic.array_access_count += 2 * arr.len() as u128;
    let mut keys: Vec<usize> = arr.iter().map(|&k| mod_key_function(k)).collect();

    counting_key_sort(arr, aux_arr, &mut counting_arr[0..divisor], &keys, false, ic);
    if max_quotient > 1 {
        ic.comparison_count += cmp::min(divisor, max_quotient) as u128 + 2; // +2 because one extra comparison for computing min
        counting_arr[..=cmp::min(divisor, max_quotient)].fill(0);

        ic.comparison_count += arr.len() as u128 + 1;
        ic.array_access_count += 2 * arr.len() as u128;
        aux_arr.iter().enumerate().for_each(|(i, &v)| keys[i] = div_key_function(v));

        counting_key_sort(aux_arr, arr, counting_arr, &keys, true, ic);
    }
}
