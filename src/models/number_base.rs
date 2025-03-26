use crate::helpers::char::{char_to_value, value_to_char};
use crate::helpers::math::{successive_division, successive_multiplication};

#[derive(Debug)]
pub struct NumberBase {
    pub integer_part: String,
    pub fractional_part: Option<String>,
    pub is_negative: bool,
    pub base: u64,
}

impl NumberBase {
    pub fn new(number_str: &str, base: u64) -> Self {
        let is_negative = number_str.starts_with('-');
        let number_str = if is_negative {
            &number_str[1..]
        } else {
            number_str
        };

        let parts: Vec<&str> = number_str.split('.').collect();
        match parts.len() {
            1 => NumberBase {
                is_negative,
                integer_part: parts[0].to_string(),
                fractional_part: None,
                base,
            },
            2 => NumberBase {
                is_negative,
                integer_part: parts[0].to_string(),
                fractional_part: Some(parts[1].to_string()),
                base,
            },
            _ => panic!("Invalid number format"),
        }
    }

    // Static conversion methods
    fn convert_integer_to_decimal(integer_str: &str, from_base: u64) -> String {
        let mut sum = 0u64;
        let chars: Vec<char> = integer_str.chars().collect();
        
        for (i, &c) in chars.iter().rev().enumerate() {
            let digit_value = char_to_value(c);
            let power = from_base.pow(i as u32);
            sum += digit_value * power;
        }
        
        sum.to_string()
    }

    fn convert_fractional_to_decimal(fractional_str: &str, from_base: u64) -> String {
        let mut result = 0.0 as f64;
        let mut power = from_base as f64;

        for c in fractional_str.chars() {
            let digit_value = char_to_value(c) as f64;
            result += digit_value / power;
            power *= from_base as f64;
        }
        // Convert to string and remove "0." prefix
        format!("{:.3}", result).trim_start_matches("0.").to_string()
    }

    fn convert_integer_from_decimal(integer_decimal_str: &str, to_base: u64) -> String {
        let integer_decimal_value = integer_decimal_str.parse::<u64>().unwrap();
        let digits = successive_division(integer_decimal_value, to_base);
        digits.iter().rev().map(|&d| value_to_char(d)).collect()
    }

    fn convert_fractional_from_decimal(fractional_decimal_str: &str, to_base: u64) -> String {
        // Use successive multiplication with max 10 digits of precision
        successive_multiplication(fractional_decimal_str, to_base, 5)
    }

    // Instance method that uses the static conversion methods
    pub fn convert_to_base(&self, to_base: u64) -> String {
        // Convert integer part to decimal
        let integer_decimal = Self::convert_integer_to_decimal(&self.integer_part, self.base);
        println!("integer_decimal: {}", integer_decimal);
        // Convert decimal integer to new base
        let integer_result = Self::convert_integer_from_decimal(&integer_decimal, to_base);

        let mut result = if let Some(fractional_part) = &self.fractional_part {
            // Convert fractional part to decimal
            let fractional_decimal = Self::convert_fractional_to_decimal(fractional_part, self.base);
            println!("fractional_decimal: {}", fractional_decimal);
            // Convert decimal fractional part to new base
            let fractional_result = Self::convert_fractional_from_decimal(&fractional_decimal, to_base);
            format!("{}.{}", integer_result, fractional_result)
        } else {
            integer_result
        };

        if self.is_negative {
            result.insert(0, '-');
        }
        result
    }
}
