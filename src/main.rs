use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn day6(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {        
        let current:String = line?; 
    }

    let result:String = format!("result");
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day6("./input/day6.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



