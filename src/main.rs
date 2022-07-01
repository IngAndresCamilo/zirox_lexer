
use std::io::{BufReader, stdin, stdout};
use std::io::prelude::*;
use std::fs::File;


/*
    read_file - Read the content of a existing file
 */
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let buff = BufReader::new(file);
    let mut buffer = String::new();

    for line in buff.lines() {
        let b = (line.unwrap() + &"".to_string());
        println!("{}", b);
    }

    Ok(buffer)
}

fn read_file_callback(command_args: Vec<String>) {
    // required argv 1
}

fn read_stdin() -> Result<u64, std::io::Error> {
    let event_return = 'EventLoop: loop {
        let command_parsed: Vec<String> = Vec::new();

        println!("Zirox Lexer 0.0.1");

        print!("Configuration Loaded successfully> ");
        stdout().flush().unwrap();
        for line in stdin().lines() {
            match line.unwrap().as_str() {
                "Exit" => break 'EventLoop,
                _ => println!("Command or binary not found"),
            }
            print!("Lexer> ");
            stdout().flush().unwrap();
        }
    };

    Ok(1)
}


fn main() -> Result<(), std::io::Error> {
    //let f = read_file("README.md")?;
    // println!("{}", f);
    read_stdin()?;
    Ok(())
}

