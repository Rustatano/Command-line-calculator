use std::error::Error;

#[derive(Clone)]
pub enum Operator {
    Add, 
    Sub, 
    Multi, 
    Div, 
}

pub struct Input {
    pub num: Vec<f64>, 
    pub ope: Vec<Operator>, 
}
impl Input {
    pub fn build(input: String)  -> Result<Input, Box<dyn Error>> {
        let input: String = input.split_whitespace().collect();
        let input: Vec<char> = input.chars().collect();

        let mut num_tmp: Vec<String> = vec![String::new()];
        let mut ope: Vec<Operator> = vec![];

        for ch in input {
            if ch == '+' {
                ope.push(Operator::Add);
                num_tmp.push(String::new());

            } else if ch == '-' {
                ope.push(Operator::Sub);
                num_tmp.push(String::new());

            } else if ch == '*' {
                ope.push(Operator::Multi);
                num_tmp.push(String::new());

            } else if ch == '/' {
                ope.push(Operator::Div);
                num_tmp.push(String::new());

            } else {
                num_tmp.last_mut().unwrap().push_str(&ch.to_string());
            }
        }

        let num = input_num(num_tmp)?;

        Ok(Input { num, ope, })
    }
}

pub fn multi_div_opes(input: &mut Input) -> Input {
    let mut ope_index = 0;
    let ope_clone = input.ope.clone();

    for op in ope_clone {
        let switch: bool;
        (input.num[ope_index], ope_index, switch) = match op {
            Operator::Multi => (
                input.num[ope_index] * input.num[ope_index + 1], 
                ope_index, 
                true,
            ), 
            Operator::Div => (
                input.num[ope_index] / input.num[ope_index + 1], 
                ope_index, 
                true,
            ), 
            _ => (
                input.num[ope_index], 
                ope_index + 1,
                false,
            ), 
        };

        if switch {
            input.ope.remove(ope_index);
            input.num.remove(ope_index + 1);
        }
    }

    Input { num: input.num.clone(), ope: input.ope.clone() }
}

pub fn add_sub_opes(input: &mut Input) -> Input {
    let mut ope_index = 0;
    let ope_clone = input.ope.clone();
    
    for op in ope_clone {
        let switch: bool;
        (input.num[ope_index], ope_index, switch) = match op {
            Operator::Add => (
                input.num[ope_index] + input.num[ope_index + 1], 
                ope_index, 
                true
            ), 
            Operator::Sub => (
                input.num[ope_index] - input.num[ope_index + 1], 
                ope_index, 
                true
            ), 
            _ => (
                input.num[ope_index], 
                ope_index + 1, 
                false
            ), 
        };

        if switch {
            input.ope.remove(ope_index);
            input.num.remove(ope_index + 1);
        }
    }

    Input { num: input.num.clone(), ope: input.ope.clone() }
}

pub fn input_num(num_tmp: Vec<String>) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut num: Vec<f64> = vec![];
    for num_index in 0..num_tmp.len() {
        num.push(num_tmp[num_index].parse()?)
    }
    Ok(num)
}

pub fn run(input: String) -> Result<(), Box<dyn Error>>{
    let mut input = Input::build(input)?;
        
    let mut input = multi_div_opes(&mut input);

    let input = add_sub_opes(&mut input);

    println!("Answer: {}", input.num[0]);

    Ok(())
}
