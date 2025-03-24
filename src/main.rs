mod helpers;
mod models;

use crate::models::number_base::NumberBase;

fn main() {
    let number = NumberBase::new("132.631", 7);
    let new_base = 3;
    println!("{}.{} in base {} is {} in base {}", number.integer_part, number.fractional_part.clone().unwrap_or_default(), number.base, number.convert_to_base(new_base), new_base);
}
