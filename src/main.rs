mod helpers;
mod models;

use crate::models::number_base::NumberBase;

fn main() {
    let number = NumberBase::new("-10", 7);
    println!("{:?}", number);
    println!("{}", number.convert_to_base(2));
}
