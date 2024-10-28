use std::fs;
use std::error::Error;
use back::
fn main() -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string("/tmp/a")?;
    println!("{}", message);
    Ok(())
}