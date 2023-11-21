use std::env;
use std::io::{Error};
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: grep WORD [File]");
        return Ok(());
    }

    let word = &args[1];
    let path = &args[2];

    let result = read_lines(path);

    let mut counter = 0;

    for eachline in result {
        counter = counter + 1;        
        if eachline.contains(word) {
            println!("In line {}: {}", counter, eachline);
        }
    }

    return Ok(());
}
