use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn day5(input_path:String) -> Result<String, Error> {
    
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
   
    for line in buffered.lines() {        
        let current:String = line?;   
    }
    
    let result:String = String::from("result!");

    Ok(result)
}

fn main() -> Result<(), Error> {
    //day1_a("day1_input.txt".to_string())?;
    //day1_b("day1_b_test.txt".to_string())?;
    //let result:String = day1_b("./input/day1_input.txt".to_string()).unwrap(); // Answer: 1516
    //let result:String = day2("./input/day2_input.txt".to_string()).unwrap();

    //let result:String = day3_b("./input/day3_input.txt".to_string()).unwrap();
    //let result:String = day3_b("./tests/day3_b_test.txt".to_string()).unwrap();

    //let result:String = day4("./input/day4.txt".to_string(), false).unwrap();

    let result:String = day5("./input/day5.txt".to_string()).unwrap();

    Ok(())
}



