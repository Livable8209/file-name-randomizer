// tetrio plus sound randomizer
// takes a directory of files, collects all the names
// then renames those files to the collection of name randomly
// (this is made for tetrio plus but it should just work fine for any other files)

use std::error::Error;
use std::path::Path;
use std::process::exit;
use std::{env, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let path = args.first().unwrap(); //this will error if no path is given :(

    let input_directory = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error hit, exiting. {}", err);
            exit(1);
        } // if we ever hit this, we better stop
    };
    let input_directory = input_directory
        .map(|result| result.map(|path| path.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    let files = input_directory.unwrap(); // if this errors, something TERRIBLE has happened

    for file in files {
        println!("{:?}", file.file_name());
    }

    if !fs::exists("randomized_sounds")? {
        println!("Making randomized_sounds directory");
        let res = fs::create_dir("randomized_sounds");
        if res.is_err() {
            println!("Caught error: {:?}", res.err());
        }
    } else {
        println!("randomized_sounds already exists");
    }

    let randomized_dir = Path::new("randomized_sounds");

    println!("{}", randomized_dir.exists());
    Ok(())
}
