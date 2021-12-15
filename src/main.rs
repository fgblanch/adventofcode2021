use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn day_n(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
    for line in buffered.lines() {        
        let current:String = line?; 
        let temp_vec:Vec<&str> = current.split(',').collect();     
    }

    let result:String = format!("result");
    
    Ok(result)
}

fn compute_fuel(crabs:Vec<u8>, position:u8)->u8{
    let mut result:u8 = 0;

    for crab in crabs{
        if crab > position{
            result += crab-position;
        }else{
            result += position-crab;
        }
    }

    result
}


fn day7(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut crabs:Vec<u8> = Vec::new();

    for line in buffered.lines() {        
        let current:String = line?; 
        let temp_vec:Vec<&str> = current.split(',').collect();     
        for number in temp_vec{
            crabs.push(number.parse().unwrap());
        }
    }

    crabs.sort();

    println!("{:?}", crabs);

    let result:String = format!("{}", compute_fuel(crabs, 10));
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day7("./tests/day7.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



