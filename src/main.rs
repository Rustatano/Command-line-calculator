use std::io;

fn main() {
    loop {
        // print!("\x1B[2J\x1b[1;1H");

        println!("Simple Calculator");
        println!("Write a math problem: ");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed reading input: {e}"),
        };

        if let Err(e) = calculator::run(input) {
            eprintln!("Application Error: {e}");
        }

        let mut end = String::new();

        match io::stdin().read_line(&mut end) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed reading input: {e}"),
        };
    }
}
