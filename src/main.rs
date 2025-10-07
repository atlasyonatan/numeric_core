use numeric_core::{numeric_core, numeric_core_sequence};
use std::env;

mod numeric_core;
mod partition;

fn main() {
    let mut args = env::args().skip(1); // skip program name

    let mut is_sequence = false;
    let mut value_arg = None;

    let arg = args.next().expect("Missing argument");
    if arg == "-s" {
        is_sequence = true;
    } else {
        value_arg = Some(arg);
    }

    if is_sequence {
        let values: Vec<u32> = args
            .take(5)
            .map(|x| x.parse().expect("Sequence values must be valid u32"))
            .collect();

        if values.len() != 4 {
            panic!("Must have exactly 4 values in the sequence");
        }
        let sequence: [u32; 4] = std::array::from_fn(|i| values[i]);
        let result = numeric_core_sequence(&sequence);

        if let Some(value) = result {
            println!("{}", value);
        } else {
            println!("No numeric core");
        }

        return;
    }

    let n: u32 = value_arg
        .unwrap()
        .parse()
        .expect("Argument must be a valid u32");

    let result = numeric_core(n, 10);

    if let Some(value) = result {
        println!("{}", value);
    } else {
        println!("No numeric core");
    }
}
