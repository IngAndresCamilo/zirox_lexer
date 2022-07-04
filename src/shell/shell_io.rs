use std::io::{BufReader, stdin, stdout};
use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;


#[derive(Debug)]
struct Action {
    action_name: String,
    action_args: Vec<String>,
    action_len: usize
}

/*
    read_file - Read the content of a existing file
*/
pub fn read_file(path: &str) -> Result<usize, std::io::Error> {
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



pub fn read_stdin() -> Result<u64, std::io::Error> {
    'EventLoop: loop {

        println!("Zirox Lexer 0.0.1");

        print!("Configuration Loaded successfully> ");
        stdout().flush().unwrap();
        for line in stdin().lines() {

            let line = line.unwrap();
            let ref1 = Rc::new(&line);
            let ref2 = Rc::new(&line);

            let cmd_parsed: Vec<String> = ref1.split(" ")
                            .map(|val| val.to_string())
                            .collect();

            if cmd_parsed.is_empty() || ref2.as_str().to_string().eq(""){
                continue;
            }

            let action = Action {
                action_name: cmd_parsed.first().unwrap().to_string(),
                action_args: Vec::from(&cmd_parsed[1..]),
                action_len: cmd_parsed.len()
            };

            println!("{:?}", action);


            match action.action_name.as_str() {

                "Read" => {
                    let b= read_file(cmd_parsed[1].as_str())?;
                    println!("- - - - - - - - - - - - -\nBytes:{}", b);
                },
                "Exit" => break 'EventLoop,
                _ => println!("Command or binary not found"),
            }
            print!("Lexer> ");
            stdout().flush().unwrap();
        }
    };

    Ok(1)
}
