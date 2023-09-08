use std::error::Error;

pub fn input_numbers(num_tmp: Vec<String>) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut num: Vec<f64> = vec![];
    for num_index in 0..num_tmp.len() {
        num.push(num_tmp[num_index].parse()?)
    }
    Ok(num)
}
