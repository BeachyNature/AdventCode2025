use std::env;
use std::fs;

fn main() {
    
    // Read in the text input file
    println!("Reading in the plain text file");
    let contents = fs::read_to_string("/mnt/c/Users/tycon/Documents/RustPrograms/advent_code/inputs/inputs.txt")
        .expect("Issues reading in text file");

    // Get inputs and split them by 2 different lines
    println!("{:?}", contents);
}
