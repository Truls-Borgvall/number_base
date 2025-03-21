pub fn char_to_value(c: char) -> u64 {
    match c {
        '0'..='9' => c as u64 - '0' as u64,
        'A'..='Z' => c as u64 - 'A' as u64 + 10,
        'a'..='z' => c as u64 - 'a' as u64 + 36,
        _ => panic!("Invalid character for number conversion"),
    }
}

pub fn value_to_char(value: u64) -> char {
    match value {
        0..=9 => (value as u8 + b'0') as char,
        10..=35 => ((value - 10) as u8 + b'A') as char,
        36..=61 => ((value - 36) as u8 + b'a') as char,
        _ => panic!("Value too large for conversion"),
    }
}