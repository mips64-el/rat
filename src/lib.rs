use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;

type RatResult<'a> = Result<(), (&'a String, io::Error)>;

pub fn run(files: &[String]) -> RatResult {
    for filename in &files[1..] {
        rat_file(filename).or_else(|e| RatResult::Err((&filename, e)))?;
    }

    Ok(())
}

fn rat_file(filename: &String) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = match file.read(&mut buffer)? {
            0 => break,
            n => n
        };
        io::stdout().write_all(&buffer[0..bytes_read])?;
    }
    Ok(())
}
