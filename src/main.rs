mod helpers;
use helpers::char::{char_to_value, value_to_char};
use helpers::math::successive_division;

#[derive(Debug)]
struct NumberBase {
    integer_part: String,
    decimal_part: Option<String>,
    from_base: u64,
}

impl NumberBase {
    fn new(number_str: &str, from_base: u64) -> Self {
        let parts: Vec<&str> = number_str.split('.').collect();
        match parts.len() {
            1 => NumberBase {
                integer_part: parts[0].to_string(),
                decimal_part: None,
                from_base,
            },
            2 => NumberBase {
                integer_part: parts[0].to_string(),
                decimal_part: Some(parts[1].to_string()),
                from_base,
            },
            _ => panic!("Invalid number format"),
        }
    }

    // Static conversion methods
    fn convert_to_decimal(number_str: &str, from_base: u64) -> u64 {
        let mut result = 0u64;
        for c in number_str.chars() {
            result = result * from_base + char_to_value(c);
        }
        result
    }

    fn convert_from_decimal(decimal_value: u64, to_base: u64) -> String {
        let digits = successive_division(decimal_value, to_base);
        digits.iter().rev().map(|&d| value_to_char(d)).collect()
    }

    // Instance method that uses the static conversion methods
    fn convert_to_base(&self, to_base: u64) -> String {
        let decimal = Self::convert_to_decimal(&self.integer_part, self.from_base);
        Self::convert_from_decimal(decimal, to_base)
    }
}

fn main() {
    let number = NumberBase::new("1010", 2);
    println!("Converting from base {} number: {}", number.from_base, number.integer_part);
    
    // Using the instance method
    let base_10 = number.convert_to_base(10);
    println!("In decimal (base 10): {}", base_10);
    
    // Using the static methods
    let decimal = NumberBase::convert_to_decimal("1010", 2);
    println!("Using static method - Decimal: {}", decimal);
    let binary = NumberBase::convert_from_decimal(decimal, 2);
    println!("Using static method - Binary: {}", binary);
}
