use std::io;

fn main() {
    loop {
        println!("Simple Calculator");
        println!("Write a math problem: ");
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e1) => eprintln!("Failed reading input: {e1}"),
        };

        if let Err(e2) = calculator::run(input) {
            eprintln!("Application Error: {e2}");
        };
        
        let mut end = String::new();
        match io::stdin().read_line(&mut end) {
            Ok(_) => (),
            Err(e3) => eprintln!("Failed reading input: {e3}"),
        };
        
        match clearscreen::clear() {
            Ok(()) => (),
            Err(e4) => eprintln!("Failed to clear screen: {e4}"),
        };
    }
}
