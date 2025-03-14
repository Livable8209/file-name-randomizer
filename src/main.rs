// tetrio plus sound randomizer
// takes a directory of files, collects all the names
// then renames those files to the collection of name randomly
// (this is made for tetrio plus but it should just work fine for any other files)

use std::error::Error;
use std::{fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let stuff = match fs::read_dir("test_files") {
        Ok(dir) => dir,
        Err(err) => panic!("We hit an error! {}", err), // if we ever hit this, we better stop
    };
    let stuff = stuff
        .map(|result| result.map(|path| path.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    let files = stuff.unwrap();

    for file in files {
        println!("{:?}", file.file_name());
    }
    Ok(())
}
