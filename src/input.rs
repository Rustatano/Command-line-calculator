use snafu::{prelude::*, Whatever};

use crate::{
    operator::Operator, 
    input_numbers::input_numbers,
};

#[derive(Clone, Debug)]
pub enum Bracket {
    Open,
    Close,
    None
}

#[derive(Debug)]
pub struct Input {
        pub numbers: Vec<f64>, 
        pub operators: Vec<Operator>, 
        pub brackets: Vec<Bracket>,
}

impl Input {
    pub fn build(input: String)  -> Result<Input, Whatever> {
        let input: String = input.split_whitespace().collect();
        let input: Vec<char> = input.chars().collect();

        let mut num_tmp: Vec<String> = vec![String::new()];
        let mut operators: Vec<Operator> = vec![];
        let mut brackets: Vec<Bracket> = vec![];

        let mut open_bracket_count = 0;
        let mut closed_bracket_count = 0;

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

            } else if ch == '(' {
                open_bracket_count += 1;

            } else if ch == ')' {
                closed_bracket_count += 1;

            } else {
                num_tmp.last_mut().unwrap().push_str(&ch.to_string());
            }
        }

        if open_bracket_count != closed_bracket_count {
            whatever!("Invalid input");
        }

        let numbers = whatever!(input_numbers(num_tmp), "Invalid input");
        
        Ok(Input { numbers, operators, brackets })
    }
}
