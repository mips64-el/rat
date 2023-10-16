use std::io;
use std::env;
use std::process;
use colored::Colorize;

macro_rules! err_msg {
    ($($s:expr),*) => {
        eprintln!("{} {}", "Error:".red().bold(), format!($($s),*));
    };
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file1> <file2> ....", args[0]);
        process::exit(1);
    }

    if let Err((filename, e)) = rat::run(&args) {
        match e.kind() {
            io::ErrorKind::NotFound => {
                err_msg!("file {} not found", filename.yellow());
            },
            _ => {}
        }

        process::exit(1);
    }
    
    Ok(())
}

