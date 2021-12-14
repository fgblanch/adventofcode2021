use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn day6(input_path:String, number_days:u32) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut lanternfish:Vec<u8> = Vec::new();

    for line in buffered.lines() {        
        let current:String = line?; 
        let temp_vec:Vec<&str> = current.split(',').collect(); 
        for number in temp_vec {
            lanternfish.push(number.parse().unwrap());
        }     
    }

    println!("Initial state {:?}",lanternfish);

    for day in 1..number_days+1{
        let mut new_lanternfish:Vec<u8> = Vec::new();
        let mut new_fish_counter = 0;

        for fish in &lanternfish{
            if *fish == 0 {
                new_lanternfish.push(6);
                new_fish_counter +=1;
            }else{
                new_lanternfish.push(*fish-1);
            }            
        }
        for _i in 0..new_fish_counter{
            new_lanternfish.push(8);
        }
        lanternfish = new_lanternfish;  
        //println!("After {} days: {:?}",day,lanternfish);      
    }

    let result:String = format!("After {} there are {} fish",number_days,lanternfish.len());
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day6("./input/day6.txt".to_string(),256).unwrap();
    println!("{}", result);

    Ok(())
}



