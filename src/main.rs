fn convert_to_base(mut number: u32, target_base: u32) -> String {
    // Handle special case for zero
    if number == 0 {
        return "0".to_string();
    }

    let mut digits = Vec::new();
    
    // Successive division algorithm
    while number > 0 {
        let remainder = number % target_base;
        // Convert remainder to character (0-9 or A-Z for bases > 10)
        let digit = if remainder < 10 {
            (remainder as u8 + b'0') as char // b'0' is the ASCII value of '0' (which is 48)
        } else {
            (remainder as u8 - 10 + b'A') as char // b'A' is the ASCII value of 'A' (which is 65)
        };
        digits.push(digit);
        number /= target_base;
    }
    
    // Reverse the digits since we collected them in reverse order
    digits.iter().rev().collect()
}

fn main() {
    // Example usage
    let number = 42;
    
    // Test with different bases
    let bases = [2, 8, 16, 20];
    for &base in &bases {
        let result = convert_to_base(number, base);
        println!("{} in base 10 = {} in base {}", number, result, base);
    }
}
