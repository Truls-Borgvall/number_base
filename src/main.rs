mod helpers;
mod models;

use crate::models::number_base::NumberBase;

fn main() {
    let number = NumberBase::new("10525.24202", 7);
    println!("{}", number.convert_to_base(2));
}
