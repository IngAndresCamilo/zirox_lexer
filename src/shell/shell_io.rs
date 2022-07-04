use std::io::{BufReader, stdin, stdout};
use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Action {
    pub action_name: String,
    pub action_args: Vec<String>,
    pub action_len: usize
}

/*
    read_file - Read the content of a existing file
*/
pub fn read_file(action: &Action) -> Result<usize, Error> {
    
    if action.action_len == 0 {
        return Err(std::io::Error::new(ErrorKind::Other, "No file path specified"))
    }

    let file = File::open(action.action_args.first().unwrap())?;
    let buff = BufReader::new(&file);
    let mut bytes_number: usize = 0;


    for line in buff.lines() {
        let b = line.unwrap() + &"".to_string();
        println!("{}", b);
        bytes_number += b.len();
    }
    Ok(bytes_number)
}



pub fn read_stdin() -> Result<u64, std::io::Error> {
    
    println!("Zirox Lexer 0.0.1");
    'EventLoop: loop {


        let mut buffer = String::new();


        print!("Lexer> ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut buffer)
            .expect("Error reading from stdin");
        
        let buffer = buffer.trim().to_string(); 
        let ref1 = Rc::new(&buffer);
        let ref2 = Rc::new(&buffer);

        let cmd_parsed: Vec<String> = ref1.split(" ")
                        .map(|val| val.to_string())
                        .collect();



        if cmd_parsed.is_empty() || ref2.as_str().to_string().eq(""){
                continue;
        }

        let action = Action {
                action_name: cmd_parsed.first().unwrap().to_string(),
                action_args: Vec::from(&cmd_parsed[1..]),
                action_len: cmd_parsed.len() - 1
        };



        match action.action_name.as_str() {

                "Read" => {
                    match read_file(&action) {
                        Ok(bytes) => println!("{}", bytes),
                        Err(e) => println!("{}", e)
                    }
                },
                "Exit" => break 'EventLoop,
                _ => println!("Command or binary not found"),
            }

    };

    Ok(1)
}
