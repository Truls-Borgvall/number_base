fn char_to_value(c: char) -> u64 {
    match c {
        '0'..='9' => c as u64 - '0' as u64,
        'A'..='Z' => c as u64 - 'A' as u64 + 10,
        'a'..='z' => c as u64 - 'a' as u64 + 36,
        _ => panic!("Invalid character for number conversion"),
    }
}

fn value_to_char(value: u64) -> char {
    match value {
        0..=9 => (value as u8 + b'0') as char,
        10..=35 => ((value - 10) as u8 + b'A') as char,
        36..=61 => ((value - 36) as u8 + b'a') as char,
        _ => panic!("Value too large for conversion"),
    }
}

fn convert_from_base(number_str: &str, from_base: u64) -> u64 {
    let mut result = 0u64;
    for c in number_str.chars() {
        result = result * from_base + char_to_value(c);
    }
    result
}

fn convert_to_base(mut number: u64, target_base: u64) -> String {
    // Handle special case for zero
    if number == 0 {
        return "0".to_string();
    }

    let mut digits = Vec::new();
    
    // Successive division algorithm
    while number > 0 {
        let remainder = number % target_base;
        digits.push(value_to_char(remainder));
        number /= target_base;
    }
    
    // Reverse the digits since we collected them in reverse order
    digits.iter().rev().collect()
}

fn convert_between_bases(number_str: &str, from_base: u64, to_base: u64) -> String {
    let decimal = convert_from_base(number_str, from_base);
    convert_to_base(decimal, to_base)
}

fn main() {
    // Example usage
    let number = "SKIPPARE";
    let from_base = 36;
    let to_base = 2;
    let result = convert_between_bases(number, from_base, to_base);
    println!("{} in base {} = {} in base {}", number, from_base, result, to_base);
    
    // Let's also print the decimal value to see the intermediate result
    let decimal = convert_from_base(number, from_base);
    println!("{} in base {} = {} in decimal", number, from_base, decimal);
}
