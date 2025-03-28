use super::char::value_to_char;

/// Konverterar en decimal bråkdel (som en sträng, t.ex. "625" för 0.625)
/// till en bråksträng i målbasen genom successiv multiplikation.
/// Använder heltalsaritmetik genom att representera decimalen som ett förhållande (täljare/nämnare)
/// för att undvika flyttalsprecision samtidigt som den följer successiv multiplikationsalgoritmen.
/// Algoritmen fungerar genom att:
/// 1. Representera decimalen som täljare/nämnare (t.ex. 625/1000 för 0.625)
/// 2. Successivt multiplicera täljaren med målbasen
/// 3. Beräkna heltalsdelen för varje steg som blir nästa siffra
/// 4. Fortsätta med resten tills antingen resten blir 0 eller max antal siffror nås
pub fn successive_multiplication(decimal_str: &str, to_base: u64, max_digits: usize) -> String {
    // Konvertera decimalsträng till heltalstäljare
    let numerator = decimal_str.parse::<u64>().unwrap();
    // Beräkna nämnaren baserat på längden av decimalsträngen (t.ex. "625" -> 1000)
    let denominator = 10u64.pow(decimal_str.len() as u32);
    
    let mut result = String::new();
    let mut current_num = numerator;
    
    for _ in 0..max_digits {
        // Multiplicera med målbasen
        current_num *= to_base;
        
        // Beräkna nästa siffra genom heltalsdivision
        let digit = current_num / denominator;
        result.push(value_to_char(digit));
        
        // Uppdatera för nästa iteration genom att ta resten
        current_num = current_num % denominator;
        
        // Om vi har nått en rest på 0 är vi klara
        if current_num == 0 {
            break;
        }
    }
    
    result
}

/// Utför successiv divisionsalgoritm för att få siffror i en målbas.
/// Returnerar en vektor av rester i omvänd ordning (minst signifikant siffra först).
/// Algoritmen fungerar genom att:
/// 1. Successivt dividera talet med målbasen
/// 2. Spara resten från varje division
/// 3. Fortsätta tills talet blir 0
/// 4. Returnera resterna i omvänd ordning för att få rätt siffror
/// Detta är den klassiska algoritmen för att konvertera mellan talbaser för heltal.
pub fn successive_division(mut number: u64, base: u64) -> Vec<u64> {
    // Om talet är 0 bryt direkt
    if number == 0 {
        return vec![0];
    }

    let mut remainders = Vec::new();
    while number > 0 {
        // Spara resten från varje division
        remainders.push(number % base);
        // Uppdatera talet genom att dividera med målbasen
        number /= base;
    }
    remainders
}