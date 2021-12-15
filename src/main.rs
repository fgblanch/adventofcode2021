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
        if *crab > position{
            result += crab-position;
        }else{
            result += position-crab;
        }
    }

    result
}

fn search_alignment(crabs:&Vec<u32>, min_pos:u32, max_pos:u32)->u32{
    let pos_result:u32;        

    println!(" Searching between {},{} ", min_pos, max_pos);

    let min_crab:u32 = crabs[min_pos as usize];
    let max_crab:u32 = crabs[max_pos as usize];

    let min_val:u32 = compute_fuel(&crabs, min_crab);
    let max_val:u32 = compute_fuel(&crabs, max_crab);


    if max_pos-min_pos>1{ // keep recursion

        println!(" Searching between {},{}  fuel: {},{}", min_pos, max_pos, min_val,max_val);
        
        let new_pos:u32 = (min_pos + max_pos) / 2;
        if min_val < max_val {
            pos_result =search_alignment(crabs, min_pos, new_pos);
        }else{
            pos_result =search_alignment(crabs, new_pos,max_pos);
        }                
    }else{                
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

    let pos_final = search_alignment(&crabs, 0, (crabs.len()-1) as u32);
    let fuel_final = compute_fuel(&crabs, crabs[pos_final as usize]);
    println!(" Alignment position: {} fuel: {}", pos_final, fuel_final);
    let result:String = format!("{}", fuel_final);
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day7("./input/day7.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



