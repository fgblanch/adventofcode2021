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

        if x1 == x2 {
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);
            for i in y_min..y_max+1{
                let key:String = format!("{},{}",x1,i);                
                let mut val:u32 = 0;
                
                if vents.get(&key) != None {
                    val = *vents.get(&key).unwrap()
                }
                
                println!("{}",key);
                vents.insert(key, val +1);
            }
        }

        if y1 == y2 {
            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            for i in x_min..x_max+1{
                let key:String = format!("{},{}",i,y1);                
                let mut val:u32 = 0;
                
                if vents.get(&key) != None {
                    val = *vents.get(&key).unwrap()
                }
                
                vents.insert(key, val +1);
            }
        }
    }

    println!("{:?}",vents);

    let mut vent_counter = 0;
    for key in vents.keys() {
        let val = *vents.get(key).unwrap();
        if val > 1 {
            vent_counter+=1;
        }
    }

    let result:String = format!("Vent counter:{}",vent_counter);
    println!("Vent counter: {}",vent_counter);

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



