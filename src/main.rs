// tetrio plus sound randomizer
// takes a directory of files, collects all the names
// then renames those files to the collection of name randomly
// (this is made for tetrio plus but it should just work fine for any other files)

use std::error::Error;
use std::fs::{self};

fn main() -> Result<(), Box<dyn Error>> {
    let stuff = fs::read_dir(".")?;
    for dirres in stuff {
        let unwrapped = dirres?;
        println!("{:?}", unwrapped.path());
    }
    Ok(())
}
