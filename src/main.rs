use numeric_core::{numeric_core, numeric_core_sequence};
use partition::ContiguousPartitionExt;
use roman_numerals::roman_to_u32;
use std::{collections::HashSet, env};

use crate::{partition::partitions_map, roman_numerals::u32_to_roman};

mod numeric_core;
mod partition;
mod roman_numerals;

fn main() {
    let mut args = env::args().skip(1); // skip program name

    let mut is_roman = false;
    let mut value_arg = None;

    while let Some(arg) = args.next() {
        if arg == "-r" {
            is_roman = true;
        } else {
            value_arg = Some(arg);
            break; // first non-flag argument
        }
    }

    let value_arg = value_arg.expect("Please provide a value as the first argument");

    if is_roman {
        let chars: Vec<char> = value_arg.chars().collect();
        let value = coat_of_arms(&chars).unwrap();
        println!("{}", value);
        return;
    } else {
        let n: u32 = value_arg.parse().expect("Argument must be a valid u32");

        let result = numeric_core(n, 10);

        if let Some(value) = result {
            println!("{}", value);
        } else {
            println!("No numeric core");
        }
    }
}

fn coat_of_arms(digits: &[char]) -> Result<u32, String> {
    let n = roman_to_u32(&digits)?;

    if digits.len() < 4 {
        return Ok(n);
    }

    let mut i: usize = 0;
    partitions_map(digits, |p| roman_to_u32(p).unwrap())
        .filter(|numbers| all_distinct(numbers))
        .inspect(|p| {
            i += 1;
            println!("\n{}:\nitems={:?}", i, p)
        })
        .filter_map(|numbers| numeric_core_sequence(&numbers))
        .inspect(|&new_number| {
            println!(
                "new_number={:?}={:?}",
                new_number,
                u32_to_roman(new_number).iter().collect::<String>()
            )
        })
        .filter_map(|new_number| numeric_core(new_number, 10))
        .inspect(|numeric_core| println!("numeric_core={:?}", numeric_core))
        .min()
        .ok_or_else(|| "No numeric core".to_string())
}

fn coat_of_arms_v2(digits: &[char]) -> Result<u32, String> {
    roman_to_u32(&digits)?;
    let mut i: usize = 0;
    partitions_map(digits, |p| roman_to_u32(p).unwrap())
        .filter(|numbers| all_distinct(numbers))
        .inspect(|p| {
            i += 1;
            println!("\n{}:\nitems={:?}", i, p)
        })
        .filter_map(|numbers| numeric_core_sequence(&numbers))
        .inspect(|&new_number| println!("new_number={:?}", new_number,))
        .map(|new_number| u32_to_roman(new_number))
        .inspect(|chars| println!("roman={:?}", chars.iter().collect::<String>()))
        .filter_map(|chars| coat_of_arms_v2(&chars).ok())
        .inspect(|numeric_core| println!("numeric_core={:?}", numeric_core))
        .min()
        .ok_or_else(|| "No numeric core".to_string())
}

/*
fn numeric_core_roman(digits: &[char]) -> Option<u32> {
    if digits.len() < 4 {
        return Some(roman_to_u32(digits).unwrap());
    }

    let mut min: Option<u32> = None;
    for partition in digits.contiguous_partitions(4).unwrap() {
        let numbers: [u32; 4] = partition
            .into_iter()
            .map(|p| roman_to_u32(p).unwrap())
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();

        print!("{:?}", numbers);

        if let Some(result) = numeric_core_sequence(&numbers) {
            let new_roman_numeral = u32_to_roman(result);
            print!("new_roman_numeral: {:?}", new_roman_numeral);
            if let Some(value) = numeric_core_roman(&new_roman_numeral) {
                print!(" = {}", value);
                min = min.map(|current| current.min(value)).or(Some(value));
            }
        }

        println!();
    }

    return min;
}
 */

fn all_distinct<T: Eq + std::hash::Hash>(slice: impl IntoIterator<Item = T>) -> bool {
    let mut seen = HashSet::new();
    for item in slice {
        if !seen.insert(item) {
            return false; // duplicate found
        }
    }
    true
}
