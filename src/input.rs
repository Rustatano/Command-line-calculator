use std::error::Error;

use crate::operator::Operator;
use crate::input_numbers::input_numbers;


pub struct Input {
        pub numbers: Vec<f64>, 
        pub operators: Vec<Operator>, 
    }

impl Input {
    pub fn build(input: String)  -> Result<Input, Box<dyn Error>> {
        let input: String = input.split_whitespace().collect();
        let input: Vec<char> = input.chars().collect();

        let mut num_tmp: Vec<String> = vec![String::new()];
        let mut operators: Vec<Operator> = vec![];

        for ch in input {
            if ch == '+' {
                operators.push(Operator::Addition);
                num_tmp.push(String::new());

            } else if ch == '-' {
                operators.push(Operator::Substraction);
                num_tmp.push(String::new());

            } else if ch == '*' {
                operators.push(Operator::Multiplication);
                num_tmp.push(String::new());

            } else if ch == '/' {
                operators.push(Operator::Division);
                num_tmp.push(String::new());

            } else {
                num_tmp.last_mut().unwrap().push_str(&ch.to_string());
            }
        }

        let numbers = input_numbers(num_tmp)?;

        Ok(Input { numbers, operators })
    }
}
