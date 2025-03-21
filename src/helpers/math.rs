use super::char::value_to_char;

/// Converts a decimal fractional part (as a string, like "625" for 0.625)
/// into a fractional string in the target base using successive multiplication.
/// Uses integer arithmetic by representing the decimal as a ratio (numerator/denominator)
/// to avoid floating-point precision errors while still following the successive
/// multiplication algorithm.
pub fn successive_multiplication(decimal_str: &str, to_base: u64, max_digits: usize) -> String {
    // Convert decimal string to an integer numerator
    let numerator = decimal_str.parse::<u64>().unwrap();
    let denominator = 10u64.pow(decimal_str.len() as u32);
    
    let mut result = String::new();
    let mut current_num = numerator;
    
    for _ in 0..max_digits {
        // Multiply by the target base
        current_num *= to_base;
        
        // Get the next digit
        let digit = current_num / denominator;
        result.push(value_to_char(digit));
        
        // Update for next iteration
        current_num = current_num % denominator;
        
        // If we've reached a remainder of 0, we're done
        if current_num == 0 {
            break;
        }
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