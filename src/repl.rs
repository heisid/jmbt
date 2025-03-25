use std::io::{stdin, stdout, Write};

pub fn repl() {
    let banner = format!("Jembut {}", env!("CARGO_PKG_VERSION"));
    println!("{}", banner);
    println!("type '.fuck' to get fucked");
    println!("type '.exit' or '.quit' to kill yourself");
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            println!("Error reading input: \n{}", e.to_string());
            std::process::exit(1);
        } else {
            if let Some(output) = cmd_router(&input.trim()) {
                println!("{}", output);
            }
        }
    }
}

fn cmd_router(cmd: &str) -> Option<&str> {
    let is_meta = cmd.starts_with(".");
    if is_meta {
        match cmd {
            ".exit" | ".quit" => std::process::exit(0),
            ".fuck" => return Some("fuck you"),
            _ => return Some("Unrecognized command"),
        }
    }
    Some("kunyuk")
}