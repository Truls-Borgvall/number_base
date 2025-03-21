use super::char::char_to_value;

/// Implements the successive multiplication algorithm for converting fractional parts.
/// For a decimal 0.abc in base B, it computes: a*B^(-1) + b*B^(-2) + c*B^(-3)
pub fn successive_multiplication(decimal_str: &str, from_base: u64) -> f64 {
    let mut result = 0.0;
    let base_f = from_base as f64;
    
    for (i, c) in decimal_str.chars().enumerate() {
        let value = char_to_value(c) as f64;
        let power = -(i as i32 + 1);
        result += value * base_f.powi(power);
    }
    result
}

/// Performs the successive division algorithm to get digits in a target base.
/// Returns a vector of remainders in reverse order (least significant digit first).
pub fn successive_division(mut number: u64, base: u64) -> Vec<u64> {
    if number == 0 {
        return vec![0];
    }

    let mut remainders = Vec::new();
    while number > 0 {
        remainders.push(number % base);
        number /= base;
    }
    remainders
}