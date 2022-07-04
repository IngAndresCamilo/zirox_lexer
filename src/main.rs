

mod shell;

use shell::shell_io;



fn main() -> Result<(), std::io::Error> {
    shell_io::read_stdin()?;
    Ok(())
}

