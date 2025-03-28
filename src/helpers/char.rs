/// Konverterar ett tecken till dess numeriska värde i ett talsystem.
/// Stödjer:
/// - Siffror 0-9 (värden 0-9)
/// - Versaler A-Z (värden 10-35)
/// - Gemener a-z (värden 36-61)
/// Detta ger stöd för talbaser upp till bas 62.
pub fn char_to_value(c: char) -> u64 {
    match c {
        '0'..='9' => c as u64 - '0' as u64,      // Konvertera siffror 0-9
        'A'..='Z' => c as u64 - 'A' as u64 + 10, // Konvertera versaler A-Z
        'a'..='z' => c as u64 - 'a' as u64 + 36, // Konvertera gemener a-z
        _ => panic!("Ogiltigt tecken för talbaskonvertering"),
    }
}

/// Konverterar ett numeriskt värde till motsvarande tecken i ett talsystem.
/// Stödjer:
/// - Värden 0-9 konverteras till siffror
/// - Värden 10-35 konverteras till versaler A-Z
/// - Värden 36-61 konverteras till gemener a-z
pub fn value_to_char(value: u64) -> char {
    match value {
        0..=9 => (value as u8 + b'0') as char,      // Konvertera till siffra
        10..=35 => ((value - 10) as u8 + b'A') as char, // Konvertera till versal
        36..=61 => ((value - 36) as u8 + b'a') as char, // Konvertera till gemen
        _ => panic!("För stort värde för konvertering"),
    }
}