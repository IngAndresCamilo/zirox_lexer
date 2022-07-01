
use std::io::{BufReader, stdin, stdout};
use std::io::prelude::*;
use std::fs::File;


/*
    read_file - Read the content of a existing file
 */
fn read_file(path: &str) -> Result<usize, std::io::Error> {
    let file = File::open(path)?;
    let buff = BufReader::new(&file);
    let mut bytes_number: usize = 0;

    println!("File path: {}\n- - - - - - - - - - - - -", path);

    for line in buff.lines() {
        let b = line.unwrap() + &"".to_string();
        println!("{}", b);
        bytes_number += b.len();
    }
    Ok(bytes_number)
}


fn read_stdin() -> Result<u64, std::io::Error> {
    let event_return = 'EventLoop: loop {

        println!("Zirox Lexer 0.0.1");

        print!("Configuration Loaded successfully> ");
        stdout().flush().unwrap();
        for line in stdin().lines() {

            let cmd_parsed: Vec<String> = line.unwrap().split(" ")
                            .map(|val| val.to_string())
                            .collect();

            match cmd_parsed.first().unwrap().as_str() {
                "Read" => {
                    let b= read_file(cmd_parsed[1].as_str())?;
                    println!("- - - - - - - - - - - - -\nBytes:{}", b);
                }
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

