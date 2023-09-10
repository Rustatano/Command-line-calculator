use snafu::{prelude::*, Whatever};

use crate::{
    operations::{
        multiplication_and_division::multiplication_and_division, 
        addition_and_substraction::addition_and_substraction,
    },
    input::Input,
};

pub fn calculate(input: String) -> Result<f64, Whatever> {
    let input: String = input.split_whitespace().collect();
    let input_char: Vec<char> = input.chars().collect(); 
    let mut input: Vec<String> = vec![];
    for ch in input_char {
        input.push(String::from(ch));
    }

    'main: while input.contains(&"(".to_string()) || input.contains(&")".to_string()){
        let mut calc: Vec<String> = vec![];
        let mut start = 0;
        let lenght = input.len();

        for ch_index in 0..lenght {
            calc.push(input[ch_index].clone());

            if input[ch_index] == "(" {
                start = ch_index;
                calc.clear();

            } else if input[ch_index] == ")" {
                calc.remove(calc.len() - 1);

                for i in (start..ch_index + 1).rev() {
                    input.remove(i);
                }
                let mut tmp_answer = whatever!(Input::build(&calc), "Invalid input");
                let mut tmp_answer = multiplication_and_division(&mut tmp_answer);
                let tmp_answer = addition_and_substraction(&mut tmp_answer);

                input.insert(start, tmp_answer.numbers[0].to_string());
                continue 'main;
            }
        }
    }

    let mut answer = whatever!(Input::build(&input), "Invalid input");
    let mut answer = multiplication_and_division(&mut answer);
    let answer = addition_and_substraction(&mut answer);

    Ok(answer.numbers[0])
}
// .clone_from_slice()