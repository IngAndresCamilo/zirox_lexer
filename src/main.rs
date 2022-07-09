

mod shell;

use shell::shell_io;
use std::env;


fn main() -> Result<(), std::io::Error> {
    let mut args = env::args();
    if args.len() < 2 {
        println!("Starting shell mode");
        shell_io::read_stdin()?;
    } else {
        let action_name = args.nth(1).unwrap();
        
        let parse_args: Vec<String> = args.map(|a| a.clone()).collect();
        let action_args = Vec::from(&parse_args[..]);
        drop(parse_args);
        
        let action_len = action_args.len();
        let action = shell_io::Action {
            action_name,
            action_args,
            action_len
        };
        shell_io::handle_action(&action);
    }
    Ok(())
}

