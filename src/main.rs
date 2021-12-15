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

fn compute_fuel( crabs:&Vec<u32>, position:u32)->u32{
    let mut result:u32 = 0;

    for crab in crabs{
        let steps:u32;
        let mut fuel:u32 = 0;

        if *crab > position{
            steps = crab-position;
        }else{
            steps = position-crab;
        }
        
        for i in 1..steps+1{
            fuel+=i;
        }

        println!("From {} to {} steps: {} fuel: {}", *crab, position, steps, fuel);
        result+=fuel;
    }

    result
}

fn search_alignment(crabs:&Vec<u32>, min_pos:u32, max_pos:u32)->u32{
    let pos_result:u32;        

    let min_val:u32 = compute_fuel(&crabs, min_pos);
    let max_val:u32 = compute_fuel(&crabs, max_pos);


    if max_pos-min_pos>1{ // keep recursion

        println!(" Searching between {},{} fuel: {},{}", min_pos, max_pos, min_val,max_val);
        
        let new_pos:u32 = ((min_pos + max_pos) as f64 / 2.0).floor() as u32;
        if min_val < max_val {
            pos_result =search_alignment(crabs, min_pos, new_pos);
        }else{
            pos_result =search_alignment(crabs, new_pos, max_pos);
        }                
    }else{                
        println!(" Searching between {},{} fuel: {},{}", min_pos, max_pos, min_val,max_val);
        if min_val < max_val {
            pos_result = min_pos;
        }else{
            pos_result = max_pos;
        }             
    }

    pos_result
}


fn day7(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut crabs:Vec<u32> = Vec::new();

    for line in buffered.lines() {        
        let current:String = line?; 
        let temp_vec:Vec<&str> = current.split(',').collect();     
        for number in temp_vec{
            crabs.push(number.trim().parse().unwrap());
        }
    }

    crabs.sort();
    println!("Length:{} crabs:{:?}", crabs.len(), crabs);

    let pos_final = search_alignment(&crabs, crabs[0], crabs[crabs.len()-1] as u32);

    let fuel_final = compute_fuel(&crabs, pos_final);
    println!(" Alignment position: {} fuel: {}", pos_final, fuel_final);
    
    let result:String = format!("{}", fuel_final);
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day7("./input/day7.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



