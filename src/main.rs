use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;
use std::collections::HashMap;


#[derive(PartialEq, Debug)]
struct Line {
    x1: u32,
    y1: u32, 
    x2: u32,
    y2: u32
}




fn day5(input_path:String) -> Result<String, Error> {
    
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
   

    let mut vents: HashMap<String, u32> = HashMap::new();

    for line in buffered.lines() {        
        let current:String = line?; 
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        let caps = re.captures(&current).unwrap();
        
        let mut x1:u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let mut y1:u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let mut x2:u32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let mut y2:u32 = caps.get(4).unwrap().as_str().parse().unwrap();

        // Resort the coordinates 
        if x1 > x2{
            for i in x2..x1+1{
                // rellenar casillas x manteniendo y
            }
        }else{
            for i in x1..x2+1{
                // rellenar casillas x manteniendo y
            }
        }

        if y1 > y2{
            for i in y2..y1+1{
                // rellenar casillas y manteniendo x
            }
        }else{
            for i in y1..y2+1{
                // rellenar casillas y manteniendo x
            }
        }

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

    let result:String = day5("./tests/day5.txt".to_string()).unwrap();

    Ok(())
}



