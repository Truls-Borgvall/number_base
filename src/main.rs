mod helpers;
use helpers::char::{char_to_value, value_to_char};
use helpers::math::{successive_division, successive_multiplication};

#[derive(Debug)]
struct NumberBase {
    integer_part: String,
    fractional_part: Option<String>,
    from_base: u64,
}

impl NumberBase {
    fn new(number_str: &str, from_base: u64) -> Self {
        let parts: Vec<&str> = number_str.split('.').collect();
        match parts.len() {
            1 => NumberBase {
                integer_part: parts[0].to_string(),
                fractional_part: None,
                from_base,
            },
            2 => NumberBase {
                integer_part: parts[0].to_string(),
                fractional_part: Some(parts[1].to_string()),
                from_base,
            },
            _ => panic!("Invalid number format"),
        }
    }

    // Static conversion methods
    fn convert_integer_to_decimal(integer_str: &str, from_base: u64) -> u64 {
        let mut result = 0 as u64;
        for c in integer_str.chars() {
            result = result * from_base + char_to_value(c);
        }
        result
    }

    fn convert_fractional_to_decimal(fractional_str: &str, from_base: u64) -> f64 {
        let mut result = 0.0 as f64;
        let mut power = from_base as f64;

        for c in fractional_str.chars() {
            let digit_value = char_to_value(c) as f64;
            result += digit_value / power;
            power *= from_base as f64;
        }
        result
    }

    fn convert_integer_from_decimal(integer_decimal_str: &str, to_base: u64) -> String {
        let integer_decimal_value = integer_decimal_str.parse::<u64>().unwrap();
        let digits = successive_division(integer_decimal_value, to_base);
        digits.iter().rev().map(|&d| value_to_char(d)).collect()
    }

    fn convert_fractional_from_decimal(fractional_decimal_str: &str, to_base: u64) -> String {
        // Use successive multiplication with max 10 digits of precision
        successive_multiplication(fractional_decimal_str, to_base, 10)
    }

    // Instance method that uses the static conversion methods
    fn convert_to_base(&self, to_base: u64) -> String {
        let integer_decimal = Self::convert_integer_to_decimal(&self.integer_part, self.from_base);
        let integer_result = Self::convert_integer_from_decimal(&integer_decimal.to_string(), to_base);
        if let Some(fractional_part) = &self.fractional_part {
            let fractional_decimal = Self::convert_fractional_to_decimal(fractional_part, self.from_base);
            let fractional_result = Self::convert_fractional_from_decimal(&fractional_decimal.to_string(), to_base);
            format!("{}.{}", integer_result, fractional_result)
        } else {
            integer_result
        }
    }
}

fn main() {
    println!("{}", NumberBase::convert_fractional_to_decimal("34", 5)); // 0.76
    println!("{}", NumberBase::convert_fractional_from_decimal("76", 5)); // 34
}
