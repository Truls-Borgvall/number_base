use super::char::value_to_char;

/// Converts a decimal fractional part (as a string, like "625" for 0.625)
/// into a fractional string in the target base using successive multiplication.
pub fn successive_multiplication(decimal_str: &str, to_base: u64, max_digits: usize) -> String {
    let mut frac: f64 = format!("0.{}", decimal_str).parse().unwrap();
    println!("{}", frac);
    let mut result = String::new();

    for _ in 0..max_digits {
        
        frac *= to_base as f64;
        let digit = frac.floor() as u64;
        result.push(value_to_char(digit));
        frac -= digit as f64;

        if frac == 0.0 {
            break;
        }
        println!("Frac: {}", frac);
        println!("Digit: {}", digit);
        println!("Result: {}", result);
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