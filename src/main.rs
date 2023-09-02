use std::io;

#[derive(Clone)]
enum Operator {    // defining Operator enum with fou variants
    Add,           // addition variant
    Sub,           // substraction variant
    Multi,         // multiplication variant
    Div,           // division variant
}

fn main() {
    println!("Simple Calculator");         // introduction
    println!("Write a math problem: ");

    let mut input = String::new();           // new empty input string
    io::stdin()                                 // user input read
    .read_line(&mut input)
    .expect("Invalid input");

    let input: String = input.split_whitespace().collect();     // deleting whitespaces in input string
    let input: Vec<char> = input.chars().collect();             // making vector of chars from input string
    let mut input_num_str: Vec<String> = vec![String::new()];   // definig number vector with strings
    let mut input_ope: Vec<Operator> = vec![];                  // defining operator vector with "Operator" enums

    for ch in input {                                                // spliting input to vector of numbers and vector of operators
        let ch = String::from(ch);                                 // changing type of ch from "char" to "String"
        if ch == "+" {                                                     // addition operator
            input_ope.push(Operator::Add);                                 // adding operator to operator vector
            input_num_str.push(String::new());                             // new empty number element/slot
        } else if ch == "-" {                                              // substraction operator
            input_ope.push(Operator::Sub);                                 // adding operator to operator vector
            input_num_str.push(String::new());                             // new empty number element/slot
        } else if ch == "*" {                                              // multiplication operator
            input_ope.push(Operator::Multi);                               // adding operator to operator vector
            input_num_str.push(String::new());                             // new empty number element/slot
        } else if ch == "/" {                                              // division operator
            input_ope.push(Operator::Div);                                 // adding operator to operator vector
            input_num_str.push(String::new());                             // new empty number element/slot
        } else {                                                           // any number
            input_num_str.last_mut().unwrap().push_str(&ch);       // adding number to previously made empty element
        }
    }

    let mut input_num: Vec<f64> = vec![];                           // defining number vector with floats
    for num_index in 0..input_num_str.len() {                // converting number vector with strings to number vector with floats
        input_num.push(match input_num_str[num_index].parse() {     // adding float to "numvec", parsing string to float 
            Ok(n) => n,                                        // Ok variant
            Err(_) => panic!("Invalid input"),                      // Err variant, panics if string cannot be parsed
        })
    }
    
    let mut ope_index = 0;                                                  // defining help index variable
    let input_ope_clone = input_ope.clone();                        // defining help clone
    for op in input_ope_clone {                                          // looping over operator vector clone
        let switch: bool;                                                          // defining help switch variable
        (input_num[ope_index], ope_index, input_ope, switch) = match op {          // matching right "Operator" variant
            Operator::Multi => (input_num[ope_index] * input_num[ope_index + 1],   // Multi variant, calculating
                ope_index,                                                         // no change to help index
                input_ope,                                                         // no change to operator vector
                true),                                                             // switch is true
            Operator::Div => (input_num[ope_index] / input_num[ope_index + 1],     // Div variant, calculating
                ope_index,                                                         // no change to help index
                input_ope,                                                         // no change to operator vector
                true),                                                             // switch is true
            _ => (input_num[ope_index],                                            // other, no calculating
                ope_index + 1,                                                     // add one to help index
                input_ope,                                                         // no change to operator vector
                false),                                                            // switch is false
        };
        if switch {
            input_ope.remove(ope_index);
            input_num.remove(ope_index + 1);
        }
    }

    let mut ope_index = 0;                                                   // defining help index variable
    let input_ope_clone = input_ope.clone();                         // defining help clone
    for op in input_ope_clone {                                           // looping over operator vector clone
        let switch: bool;                                                           // defining help switch variable
        (input_num[ope_index], ope_index, input_ope, switch) = match op {           // matching right "Operator" variant
            Operator::Add => (input_num[ope_index] + input_num[ope_index + 1],      // Add variant, calculating
                ope_index,                                                          // no change to help index
                input_ope,                                                          // no change to operator vector
                true),                                                              // switch is true
            Operator::Sub => (input_num[ope_index] - input_num[ope_index + 1],      // Sub variant, calculating
                ope_index,                                                          // no change to help index
                input_ope,                                                          // no change to operator vector
                true),                                                              // switch is true
            _ => (input_num[ope_index],                                             // other, no calculating
                ope_index + 1,                                                      // add one to help index
                input_ope,                                                          // no change to operator vector
                false),                                                             // switch is false
        };
        if switch {                                     // if switch is true
            input_ope.remove(ope_index);                // remove now used operator
            input_num.remove(ope_index + 1);     // remove second number in calculation
        }
    }
    println!("Answer: {}", input_num[0]);               // printing answer
}
