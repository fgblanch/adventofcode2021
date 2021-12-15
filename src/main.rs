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


fn day8(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
    let mut digit_counter = 0;

    for line in buffered.lines() {        
        let current:String = line?; 
        let temp_vec:Vec<&str> = current.split('|').collect();     

        let signal_patterns_string = String::from(temp_vec[0].trim());
        let digits_string = String::from(temp_vec[1].trim());

        println!("{}", signal_patterns_string);
        println!("{}", digits_string);

        let digits_temp:Vec<&str> = digits_string.split(' ').collect();     
        

        for digit in digits_temp{
            if (digit.trim().len() == 2) | (digit.trim().len() == 3) | (digit.trim().len() == 7) | (digit.trim().len() == 4) {
                digit_counter+=1
            }
        }

    }

    let result:String = format!("{}", digit_counter);
    
    Ok(result)
}


fn main() -> Result<(), Error> {
    let result:String = day8("./input/day8.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



