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
        eprintln!("{} {} <file1> ...", "Usage:".green().bold(), args[0].bold());
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

