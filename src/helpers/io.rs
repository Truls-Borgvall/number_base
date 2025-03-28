use std::io::{self, Write};
use crate::models::number_base::NumberBase;

#[derive(Debug)]
pub enum InputError {
    InvalidNumber(String),
    InvalidBase(String),
    ParseError(String),
}

pub fn get_user_input() -> Result<(NumberBase, u64), InputError> {
    let mut input = String::new();
    let mut base = String::new();
    let mut new_base = String::new();

    // Hämta talet
    print!("Ange talet: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // Hämta nuvarande bas
    print!("Ange nuvarande bas (2-36): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base).unwrap();
    let base: u64 = base.trim().parse()
        .map_err(|_| InputError::ParseError("Basen måste vara ett heltal".to_string()))?;

    // Validera bas
    if base < 2 || base > 36 {
        return Err(InputError::InvalidBase(
            format!("Basen måste vara mellan 2 och 36, men du angav {}", base)
        ));
    }

    // Hämta målbas
    print!("Ange målbas (2-36): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_base).unwrap();
    let new_base: u64 = new_base.trim().parse()
        .map_err(|_| InputError::ParseError("Basen måste vara ett heltal".to_string()))?;

    // Validera målbas
    if new_base < 2 || new_base > 36 {
        return Err(InputError::InvalidBase(
            format!("Basen måste vara mellan 2 och 36, men du angav {}", new_base)
        ));
    }

    // Validera talet
    if input.is_empty() {
        return Err(InputError::InvalidNumber("Talet kan inte vara tomt".to_string()));
    }

    // Skapa NumberBase
    let number = NumberBase::new(input, base);
    Ok((number, new_base))
} 