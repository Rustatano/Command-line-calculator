use std::error::Error;

use crate::operations::{
    multiplication_and_division::multiplication_and_division, 
    addition_and_substraction::addition_and_substraction
};
use crate::input::Input;

pub fn calculate(input: String) -> Result<(), Box<dyn Error>>{
    let mut input = Input::build(input)?;
        
    let mut input = multiplication_and_division(&mut input);

    let input = addition_and_substraction(&mut input);

    println!("Answer: {}", input.numbers[0]);

    Ok(())
}
