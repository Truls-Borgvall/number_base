mod helpers;
mod models;

use crate::helpers::io::{get_user_input, InputError};

fn main() {
    match get_user_input() {
        Ok((number, new_base)) => {
            let converted_number = number.convert_to_base(new_base);
            println!("{}{}.{} i bas {} är {} i bas {}", 
                if number.is_negative { "-" } else { "" },
                number.integer_part, 
                number.fractional_part.unwrap_or_default(), 
                number.base, 
                converted_number, 
                new_base
            );
        }
        Err(e) => {
            match e {
                InputError::InvalidNumber(msg) => println!("Fel: {}", msg),
                InputError::InvalidBase(msg) => println!("Fel: {}", msg),
                InputError::ParseError(msg) => println!("Fel: {}", msg),
            }
        }
    }
}
