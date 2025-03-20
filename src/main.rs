fn char_to_value(c: char) -> u32 {
    match c {
        '0'..='9' => c as u32 - '0' as u32,
        'A'..='Z' => c as u32 - 'A' as u32 + 10,
        'a'..='z' => c as u32 - 'a' as u32 + 36,
        _ => panic!("Invalid character for number conversion"),
    }
}

fn convert_from_base(number_str: &str, from_base: u32) -> u32 {
    let mut result = 0;
    for c in number_str.chars() {
        result = result * from_base + char_to_value(c);
    }
    result
}

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

fn convert_between_bases(number_str: &str, from_base: u32, to_base: u32) -> String {
    let decimal = convert_from_base(number_str, from_base);
    convert_to_base(decimal, to_base)
}

fn main() {
    // Example usage
    let number = "EDWARD";
    let from_base = 10;
    let to_base = 2;
    let result = convert_between_bases(number, from_base, to_base);
    println!("{} in base {} = {} in base {}", number, from_base, result, to_base);
}
