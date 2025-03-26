mod helpers;
mod models;

use crate::models::number_base::NumberBase;

fn main() {
    let number = NumberBase::new("-221.201", 3);
    let base = number.base;
    let integer_part = number.integer_part.clone();
    let fractional_part = number.fractional_part.clone().unwrap_or_default();

    let new_base = 16;
    let converted_number = number.convert_to_base(new_base);

    println!("{}{}.{} in base {} is {} in base {}", 
        if number.is_negative { "-" } else { "" },
        integer_part, 
        fractional_part, 
        base, 
        converted_number, 
        new_base
    );
}
