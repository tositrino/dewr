//! This module contains functions for the frequency counter lab
//! # Examples:
//! ````
//! use module_01::freq_count::*;
//! freq_count();
//! ````

use std::collections::HashMap;

/// This function counts the frequency of each number in the vector.
/// # Examples:
/// ````
/// use module_01::freq_count::logic;
/// let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
/// let frequencies = logic(numbers);
/// ````

pub fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}


/// This function prints the language weighing results
/// # Examples:
/// ````
/// use module_01::freq_count::freq_count;
/// freq_count();
/// ````
pub fn freq_count()->i32 {
    let result :i32 = 0;
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let frequencies = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        frequencies
    );
    result
}
