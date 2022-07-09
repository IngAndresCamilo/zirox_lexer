use dns_lookup::lookup_host;
use std::thread;
use std::net::TcpStream;
use std::io::{BufReader, stdin, stdout};
use std::io::prelude::*;
use std::fs::File;
use std::fs;
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
        return Err(std::io::Error::new(ErrorKind::Other, "IO/ERROR: No file path specified"))
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
/*
   dir_list - list directories of the system
*/
pub fn dir_list(action: &Action) -> Result<usize, Error> {
    if action.action_len == 0 {
        return Err(std::io::Error::new(ErrorKind::Other, "IO/ERROR: No directories given"))
    }

    let mut n_directories: usize = 0;

    for arg in action.action_args.iter() {
        let dirs = fs::read_dir(arg.as_str()).unwrap();
        for dir in dirs {
            println!("{}==> {}", n_directories, dir.unwrap().path().display());
            n_directories += 1;
        }
        println!("---");
    }
    Ok(n_directories)
}



pub fn resolve_dns(action: &Action) -> Result<(), std::io::Error> {

    if action.action_len == 0 {
        return Err(std::io::Error::new(ErrorKind::Other, "  I/0 No DNS given as argument"));
    }

    for dns in action.action_args.iter() {
        let hostname = dns.as_str();
        let resolver = lookup_host(hostname);
        if resolver.is_err() {
            return Err(std::io::Error::new(ErrorKind::Other, format!("I/O Error solving dns: {hostname}")))
        }
        let resolver = resolver.unwrap();
        let ip = &resolver[0];
        let mask = &resolver[1];
        println!("\t-------------------------------------------------");
        println!("\t{hostname}\n\t{ip}\n\t{mask}");
        println!("\t-------------------------------------------------")
    }
    Ok(())
}

//Receive an action structure
pub fn handle_action(action: &Action) -> bool {
    let mut exit_handler = false;
    match action.action_name.as_str() {
        "Read" => {
            match read_file(&action) {
                Ok(bytes) => println!("{}", bytes),
                Err(e) => println!("{}", e)
            }
        },
        "Solve" => {
            match resolve_dns(&action) {
                Ok(_) => println!("\t[!] Domain solving finished"),
                Err(e) => println!("{}", e)
            }
        },
        "Dir" => {
            match dir_list(&action) {
                Ok(n) => println!("{} found", n),
                Err(e) => println!("{}", e)
            }
        },
        "Exit" => exit_handler = true,
        _ => println!("Command or binary not found"),
    }
    exit_handler
}


pub fn read_stdin() -> Result<u64, std::io::Error> {
    
    println!("Zirox Lexer 0.0.1");
    'EventLoop: loop {


        let mut buffer = String::new();


        print!("<Lexer>> ");
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

        if handle_action(&action) {
            break 'EventLoop;
        }
    };
    Ok(1)
}
