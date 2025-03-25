use std::io::{stdin, stdout, Write};

pub fn repl() {
    let banner = format!("Jembut {}", env!("CARGO_PKG_VERSION"));
    println!("{}", banner);
    println!("type 'fuck' to get fucked");
    println!("type 'exit' or 'quit' to kill yourself");
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        // let mut output = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            println!("Error reading input: \n{}", e.to_string());
            std::process::exit(1);
        } else {
            match input.trim() {
                "exit" | "quit" => {
                    std::process::exit(0);
                },
                "fuck" => {
                    println!("fuck you");
                }
                _ => {
                    println!("Unrecognized command");
                },
            }
        }
    }
}