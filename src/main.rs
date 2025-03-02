use std::fs;

fn main() {
    
    // Read in the text input file
    let mut right: Vec<i32> = Vec::new();
    let mut left: Vec<i32> = Vec::new();

    // Read in the file and split into left and right values
    println!("Reading in the plain text file");
    for line in fs::read_to_string("inputs.txt").unwrap().lines(){
        let val: Vec<&str> = line.split_whitespace().collect();
        right.push(val[0].parse().expect("Unable to parse!"));
        left.push(val[1].parse().expect("Unable to parse!"));
    }
    
    // Sort as its supposed to go in order
    right.sort();
    left.sort();

    // Get the differences
    let mut diff: Vec<i32> = Vec::new();
    for idx in 0..right.len(){
        diff.push((right[idx] - left[idx]).abs());
    }

    // Sum all of the values of the differences
    let results: i32 = diff.iter().sum();
    println!("{:?}", results);
}
