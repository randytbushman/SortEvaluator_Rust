use std::cmp;
use crate::utils::{find_min_max, InstructionCounter};
use crate::algorithms::countingsort::counting_key_sort;


pub fn sort(arr: &mut [i32], mut divisor: usize, min_val_zero: bool, use_bitwise: bool, mut instruction_counter: InstructionCounter) {
    if divisor == 0 { return }; // Throw error

    let mut aux_arr = vec![0i32; arr.len()];
    let (min_val, max_val) = find_min_max(&arr);
    let max_quotient = ((max_val - min_val) as usize / divisor) + 1;

    let k = if divisor > max_quotient { divisor + 1 } else { max_quotient };
    let mut counting_arr: Vec<usize> = vec![0usize; k];

    let mod_key_function: Box<dyn Fn(i32) -> usize>;
    let div_key_function: Box<dyn Fn(i32) -> usize>;
    let d = divisor as i32;
    let p;

    if use_bitwise {
        if divisor <= 0 || (divisor & (divisor - 1)) != 0 {  // Cannot use bitwise since divisor is not power of 2
            instruction_counter.algorithm_error = true;
            return;
        }
        p = divisor.ilog2() as i32;
        if min_val_zero {
            mod_key_function = Box::new(|x: i32| ((x & (d - 1)) as usize));
            div_key_function = Box::new(|x: i32| ((x >> p) as usize));
        } else {
            mod_key_function = Box::new(|x: i32| (((x - min_val) & (d - 1)) as usize));
            div_key_function = Box::new(|x: i32| (((x - min_val) >> p) as usize));
        }
    } else {
        if min_val_zero {
            mod_key_function = Box::new(|x: i32| ((x % d) as usize));
            div_key_function = Box::new(|x: i32| ((x / d) as usize));
        } else {
            mod_key_function = Box::new(|x: i32| (((x - min_val) % d) as usize));
            div_key_function = Box::new(|x: i32| (((x - min_val) / d) as usize));
        }
    }
    qr_sort(arr, &mut aux_arr, &mut counting_arr, divisor, max_quotient, mod_key_function, div_key_function, instruction_counter);
}


fn qr_sort<E, F>(arr: &mut [i32], aux_arr: &mut [i32], counting_arr: &mut [usize], divisor: usize, max_quotient: usize, mod_key_function: E, div_key_function: F, instruction_counter: InstructionCounter)
where E: Fn(i32) -> usize,
      F: Fn(i32) -> usize,
{
    let mut keys: Vec<usize> = arr.iter().map(|&k| mod_key_function(k)).collect();
    counting_key_sort(arr, aux_arr, &mut counting_arr[0..divisor], &keys, false);
    if max_quotient > 1 {
        counting_arr[..=cmp::min(divisor, max_quotient)].fill(0);
        aux_arr.iter().enumerate().for_each(|(i, &v)| keys[i] = div_key_function(v));
        counting_key_sort(aux_arr, arr, counting_arr, &keys, true);
    }
}
