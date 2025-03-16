// tetrio plus sound randomizer
// takes a directory of files, collects all the names
// then renames those files to the collection of name randomly
// (this is made for tetrio plus but it should just work fine for any other files)

// ok i know this code is terrible but, in my defense
// im learning so yeah

use rand::prelude::*;
use std::error::Error;
use std::path::Path;
use std::{env, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let default_path = String::from(".");
    let path = args.first().unwrap_or(&default_path);

    let input_directory = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            panic!("Error hit, exiting. {}", err);
        } // if we ever hit this, we better stop
    };
    let input_directory = input_directory
        .map(|result| result.map(|path| path.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    //This will always write to your current working dir since im too lazy to implement user inputted dirs
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

    let files = input_directory.unwrap(); // if this errors, something TERRIBLE has happened
    let mut file_names: Vec<String> = Vec::with_capacity(files.len());

    for file in &files {
        file_names.push(String::from(file.file_name().unwrap().to_string_lossy())); // wow that is just terrible and bad
    }

    file_names.shuffle(&mut rng);

    for file in &files {
        let end_path = {
            let rand_file_name = file_names.pop().unwrap();
            println!(
                "Renamed {} to {}",
                file.file_name().unwrap().to_string_lossy(),
                &rand_file_name
            );
            format!("{}/{}", randomized_dir.to_string_lossy(), rand_file_name)
        };

        let file_copy = fs::copy(file, end_path);
        if file_copy.is_err() {
            println!(
                "Hit err on trying to write file! {}",
                file_copy.err().unwrap()
            )
        }
    }

    Ok(())
}
