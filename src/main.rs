use std::io;

fn main() {
    loop {
        println!("Simple Calculator");
        println!("Write a math problem: ");
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e2) => eprintln!("Failed reading input: {e2}"),
        };

        if let Err(e3) = calculator::run(input) {
            eprintln!("Application Error: {e3}");
        };
        
        let mut end = String::new();
        match io::stdin().read_line(&mut end) {
            Ok(_) => (),
            Err(e4) => eprintln!("Failed reading input: {e4}"),
        };
        
        match clearscreen::clear() {
            Ok(()) => (),
            Err(e1) => eprintln!("Failed to clear screen: {e1}"),
        };
    }
}
