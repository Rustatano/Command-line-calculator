use snafu::{prelude::*, Whatever};

use crate::{
    operations::{
        multiplication_and_division::multiplication_and_division, 
        addition_and_substraction::addition_and_substraction,
    },
    input::Input,
};

pub fn calculate(input: String) -> Result<f64, Whatever> {
    let mut input = whatever!(Input::build(input), "Invalid input");
    
    let mut input = multiplication_and_division(&mut input);
    
    let input = addition_and_substraction(&mut input);
    println!("{:?}", input);
    Ok(input.numbers[0])
}
// .clone_from_slice()