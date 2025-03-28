use crate::helpers::char::{char_to_value, value_to_char};
use crate::helpers::math::{successive_division, successive_multiplication};

/// Representerar ett tal i en specifik talbas med stöd för:
/// - Heltalsdel
/// - Valfri bråkdel
/// - Negativa tal
/// - Olika talbaser (upp till bas 62)
#[derive(Debug)]
pub struct NumberBase {
    pub integer_part: String,
    pub fractional_part: Option<String>,
    pub is_negative: bool,
    pub base: u64,
}

impl NumberBase {
    /// Skapar en ny NumberBase från en strängrepresentation av ett tal.
    /// Hanterar negativa tal, decimalpunkter och olika talbaser.
    pub fn new(number_str: &str, base: u64) -> Self {
        // Hantera negativa tal genom att extrahera tecknet
        let is_negative = number_str.starts_with('-');
        let number_str = if is_negative {
            &number_str[1..]
        } else {
            number_str
        };

        // Dela upp talet i heltals- och bråkdel
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
            _ => panic!("Ogiltigt nummerformat"),
        }
    }

    /// Konverterar en heltalsdel från källbasen till decimal.
    /// Använder potensutveckling: Σ(digit * base^position)
    /// Exempel: "1A" i bas 16 = 1*16^1 + 10*16^0 = 26
    fn convert_integer_to_decimal(integer_str: &str, from_base: u64) -> String {
        let mut sum = 0u64;
        let chars: Vec<char> = integer_str.chars().collect();
        
        // Iterera från höger till vänster för att hantera positioner korrekt
        for (i, &c) in chars.iter().rev().enumerate() {
            let digit_value = char_to_value(c);
            let power = from_base.pow(i as u32);
            sum += digit_value * power;
        }
        
        sum.to_string()
    }

    /// Konverterar en bråkdel från källbasen till decimal.
    /// Använder negativa potenser: Σ(digit * base^-position)
    /// Exempel: "A" i bas 16 = 10*16^-1 = 0.625
    fn convert_fractional_to_decimal(fractional_str: &str, from_base: u64) -> String {
        let mut result = 0.0 as f64;
        let mut power = from_base as f64;

        for c in fractional_str.chars() {
            let digit_value = char_to_value(c) as f64;
            result += digit_value / power;
            power *= from_base as f64;
        }
        // Formatera till 3 decimaler och ta bort ledande nollor
        format!("{:.3}", result).trim_start_matches("0.").to_string()
    }

    /// Konverterar en decimal heltalsdel till målbasen.
    /// Använder successiv division för att få siffrorna i rätt ordning.
    fn convert_integer_from_decimal(integer_decimal_str: &str, to_base: u64) -> String {
        let integer_decimal_value = integer_decimal_str.parse::<u64>().unwrap();
        let digits = successive_division(integer_decimal_value, to_base);
        // Vänd på siffrorna för att få rätt ordning (högst signifikant siffra först)
        digits.iter().rev().map(|&d| value_to_char(d)).collect()
    }

    /// Konverterar en decimal bråkdel till målbasen.
    /// Använder successiv multiplikation för att få siffrorna i rätt ordning.
    fn convert_fractional_from_decimal(fractional_decimal_str: &str, to_base: u64) -> String {
        successive_multiplication(fractional_decimal_str, to_base, 5)
    }

    /// Konverterar talet från källbasen till målbasen.
    /// Processen:
    /// 1. Konvertera heltalsdel till decimal
    /// 2. Konvertera bråkdel till decimal (om den finns)
    /// 3. Konvertera decimalheltal till målbas
    /// 4. Konvertera decimalbråk till målbas (om den finns)
    /// 5. Kombinera delarna och lägg till negativt tecken om nödvändigt
    pub fn convert_to_base(&self, to_base: u64) -> String {
        let integer_decimal = Self::convert_integer_to_decimal(&self.integer_part, self.base);
        let integer_result = Self::convert_integer_from_decimal(&integer_decimal, to_base);

        let mut result = if let Some(fractional_part) = &self.fractional_part {
            let fractional_decimal = Self::convert_fractional_to_decimal(fractional_part, self.base);
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
