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

pub fn split_input(input: &Vec<char>)  -> Input {
    let mut num_tmp: Vec<String> = vec![String::new()];
    let mut ope: Vec<Operator> = vec![];

    for ch in input {
        if *ch == '+' {
            ope.push(Operator::Add);
            num_tmp.push(String::new());
        } else if *ch == '-' {
            ope.push(Operator::Sub);
            num_tmp.push(String::new());
        } else if *ch == '*' {
            ope.push(Operator::Multi);
            num_tmp.push(String::new());
        } else if *ch == '/' {
            ope.push(Operator::Div);
            num_tmp.push(String::new());
        } else {
            num_tmp.last_mut().unwrap().push_str(&ch.to_string());
        }
    }

    let mut num: Vec<f64> = vec![];
    for num_index in 0..num_tmp.len() {
        num.push(match num_tmp[num_index].parse() {
            Ok(n) => n, 
            Err(_) => panic!("Invalid input"), 
        })
    }

    Input { num, ope, }
}
