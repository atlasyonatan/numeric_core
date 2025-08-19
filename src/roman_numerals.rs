use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref ROMAN_VALUES: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('I', 1);
        m.insert('V', 5);
        m.insert('X', 10);
        m.insert('L', 50);
        m.insert('C', 100);
        m.insert('D', 500);
        m.insert('M', 1000);
        m
    };

    // Precomputed descending mapping for encoding
    static ref ROMAN_ENCODING: Vec<(u32, &'static str)> = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
}
pub fn roman_to_u32(roman: &[char]) -> Result<u32, String> {
    if roman.is_empty() {
        return Err("Empty Roman numeral".to_string());
    }

    let mut total = 0;
    let mut i = 0;

    while i < roman.len() {
        let current = ROMAN_VALUES
            .get(&roman[i])
            .ok_or_else(|| format!("Invalid Roman numeral: {}", roman[i]))?;

        if i + 1 < roman.len() {
            let next = ROMAN_VALUES
                .get(&roman[i + 1])
                .ok_or_else(|| format!("Invalid Roman numeral: {}", roman[i + 1]))?;

            if current < next {
                total += next - current;
                i += 2;
                continue;
            }
        }

        total += current;
        i += 1;
    }

    Ok(total)
}

pub fn u32_to_roman(mut num: u32) -> Vec<char> {
    let mut result = Vec::new();

    for &(value, symbol) in ROMAN_ENCODING.iter() {
        while num >= value {
            num -= value;
            result.extend(symbol.chars());
        }
    }

    result
}
#[cfg(test)]
mod roman_to_u32_tests {
    use super::*;

    #[test]
    fn test_roman_to_u32() {
        // Basic numbers
        assert_eq!(roman_to_u32(&['I']).unwrap(), 1);
        assert_eq!(roman_to_u32(&['V']).unwrap(), 5);
        assert_eq!(roman_to_u32(&['X', 'X']).unwrap(), 20);

        // Subtractive notation
        assert_eq!(roman_to_u32(&['I', 'V']).unwrap(), 4);
        assert_eq!(roman_to_u32(&['I', 'X']).unwrap(), 9);
        assert_eq!(roman_to_u32(&['X', 'L']).unwrap(), 40);
        assert_eq!(roman_to_u32(&['C', 'M']).unwrap(), 900);

        // Multiple numerals
        assert_eq!(roman_to_u32(&['X', 'I', 'V']).unwrap(), 14);
        assert_eq!(roman_to_u32(&['X', 'I', 'I', 'I']).unwrap(), 13);
        assert_eq!(
            roman_to_u32(&['M', 'C', 'M', 'X', 'C', 'I', 'V']).unwrap(),
            1994
        ); // MCMXCIV

        // Invalid character
        let invalid = ['A', 'I'];
        assert!(roman_to_u32(&invalid).is_err());

        // Empty input
        let empty: [char; 0] = [];
        assert!(roman_to_u32(&empty).is_err());
    }
}

#[cfg(test)]
mod u8_to_roman_tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(u32_to_roman(1), vec!['I']);
        assert_eq!(u32_to_roman(4), vec!['I', 'V']);
        assert_eq!(u32_to_roman(9), vec!['I', 'X']);
    }

    #[test]
    fn test_compound() {
        assert_eq!(u32_to_roman(13), vec!['X', 'I', 'I', 'I']);
        assert_eq!(u32_to_roman(40), vec!['X', 'L']);
        assert_eq!(u32_to_roman(58), vec!['L', 'V', 'I', 'I', 'I']);
    }

    #[test]
    fn test_larger() {
        assert_eq!(u32_to_roman(90), vec!['X', 'C']);
        assert_eq!(u32_to_roman(99), vec!['X', 'C', 'I', 'X']);
        assert_eq!(u32_to_roman(255), vec!['C', 'C', 'L', 'V']);
        assert_eq!(u32_to_roman(2024), vec!['M', 'M', 'X', 'X', 'I', 'V']);
    }
}
