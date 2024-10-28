use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string("/tmp/a")?;
    println!("{}", message);
    Ok(())
}