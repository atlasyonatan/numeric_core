use numeric_core::{numeric_core, numeric_core_sequence};
use partition::ContiguousPartitionExt;
use roman_numerals::roman_to_u32;
use std::{collections::HashSet, env};

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
        let value = numeric_core_roman(&chars, |numbers| all_distinct(numbers)).unwrap();
        println!("{}", value);
        return;
    }

    let n: u32 = value_arg.parse().expect("Argument must be a valid u32");

    let result = numeric_core(n, 10);

    if let Some(value) = result {
        println!("{}", value);
    } else {
        println!("No numeric core");
    }
}

fn numeric_core_roman(digits: &[char], predicate: impl Fn(&[u32]) -> bool) -> Result<u32, String> {
    roman_to_u32(&digits)?;

    if digits.len() < 4 {
        return Err("Not enough roman numerals".to_string());
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

        if !predicate(&numbers) {
            continue;
        }

        print!("{:?}", numbers);

        if let Some(result) = numeric_core_sequence(&numbers) {
            if let Some(value) = numeric_core(result, 10) {
                print!(" = {}", value);
                min = min.map(|current| current.min(value)).or(Some(value));
            } else {
                print!(" no numeric core");
            }
        }

        println!();
    }

    return min.ok_or_else(|| "No numeric core".to_string());
}

fn all_distinct<T: Eq + std::hash::Hash>(slice: impl IntoIterator<Item = T>) -> bool {
    let mut seen = HashSet::new();
    for item in slice {
        if !seen.insert(item) {
            return false; // duplicate found
        }
    }
    true
}
