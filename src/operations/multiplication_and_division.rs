use crate::{
    input::Input, 
    operator::Operator
};

pub fn multiplication_and_division(input: &mut Input) -> Input {
    let mut ope_index = 0;
    let ope_clone = input.operators.clone();

    for op in ope_clone {
        let switch: bool;
        (input.numbers[ope_index], switch) = match op {
            Operator::Multiplication => (
                input.numbers[ope_index] * input.numbers[ope_index + 1], 
                true,
            ), 
            Operator::Division => (
                input.numbers[ope_index] / input.numbers[ope_index + 1], 
                true,
            ), 
            _ => (
                input.numbers[ope_index], 
                false,
            ), 
        };

        if switch {
            input.operators.remove(ope_index);
            input.numbers.remove(ope_index + 1);
        } else {
            ope_index += 1;
        }
    }

    Input { numbers: input.numbers.clone(), operators: input.operators.clone(), brackets: input.brackets.clone()}
}
