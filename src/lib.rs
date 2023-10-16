use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;

type CatResult<'a> = Result<(), (&'a String, io::Error)>;

pub fn run_cat(files: &[String]) -> CatResult {
    for filename in &files[1..] {
        cat_file(filename).or_else(|e| CatResult::Err((&filename, e)))?;
    }

    Ok(())
}

fn cat_file(filename: &String) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?; 
        if bytes_read == 0 {
            break;
        }
        io::stdout().write_all(&buffer[0..bytes_read])?;
    }
    Ok(())
}