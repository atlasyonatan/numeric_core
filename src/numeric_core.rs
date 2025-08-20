use crate::partition::partitions_map;

pub fn numeric_core(number: u32, radix: u32) -> Option<u32> {
    let digits = get_digits(number, radix);
    if digits.len() < 4 {
        return Some(number);
    }

    partitions_map(&digits, |p| to_number(&p, radix).unwrap())
        .filter_map(|partition| numeric_core_sequence(&partition))
        .filter_map(|new_number| numeric_core(new_number, radix))
        .min()
}

fn to_number(digits: &[char], radix: u32) -> Option<u32> {
    let mut value = 0u32;

    for &c in digits {
        // Convert char to digit in the given radix
        let digit = c.to_digit(radix)?;
        // Check for overflow
        value = value.checked_mul(radix)?.checked_add(digit)?;
    }

    Some(value)
}

fn get_digits(mut x: u32, radix: u32) -> Vec<char> {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

#[derive(Clone, Copy)]
enum Operation {
    Subtraction,
    Multiplication,
    Division,
}

fn operate(op: Operation, rhs: f32, lhs: f32) -> f32 {
    match op {
        Operation::Subtraction => rhs - lhs,
        Operation::Multiplication => rhs * lhs,
        Operation::Division => rhs / lhs,
    }
}

pub fn numeric_core_sequence(sequence: &[u32; 4]) -> Option<u32> {
    numeric_core_recursive(
        &sequence[1..].try_into().unwrap(),
        [
            Operation::Subtraction,
            Operation::Multiplication,
            Operation::Division,
        ],
        0,
        sequence[0] as f32,
    )
}

fn numeric_core_recursive<const N: usize>(
    numbers: &[u32; N],
    mut operations: [Operation; N],
    depth: usize,
    current_val: f32,
) -> Option<u32> {
    if depth == N {
        if current_val.is_sign_negative() || current_val.fract() != 0.0 {
            return None;
        }
        return Some(current_val as u32);
    }

    let next_number = numbers[depth] as f32;

    let mut min: Option<u32> = None;
    // Iterate over remaining operations (by swapping in place)
    for i in depth..N {
        // Swap current op into "depth" position
        operations.swap(depth, i);

        let op = operations[depth];
        let new_val = operate(op, current_val, next_number);

        // Recurse deeper
        let result = numeric_core_recursive(numbers, operations, depth + 1, new_val);

        if let Some(value) = result {
            min = min.map(|current| current.min(value)).or(Some(value));
        }

        // Swap back (backtrack)
        operations.swap(depth, i);
    }

    return min;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 10, Some(1))]
    #[case(12, 10, Some(12))]
    #[case(123, 10, Some(123))]
    #[case(1213, 10, None)]
    #[case(3614, 10, Some(14))]
    #[case(86455, 10, Some(18))]
    #[case(1111111, 10, Some(0))]
    fn test_numeric_core(#[case] number: u32, #[case] radix: u32, #[case] expected: Option<u32>) {
        assert_eq!(numeric_core(number, radix), expected);
    }
}
