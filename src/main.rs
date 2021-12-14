use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashMap;

fn day6(input_path:String, number_days:u32) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut lanternfish: HashMap<u8, u64> = HashMap::new();

    for line in buffered.lines() {        
        let current:String = line?; 
        let temp_vec:Vec<&str> = current.split(',').collect(); 
        for number in temp_vec {            
            let age:u8 = number.parse().unwrap();

            if lanternfish.get(&age) != None {
                *lanternfish.get_mut(&age).unwrap() +=1;
            }else{
                lanternfish.insert(age, 1);
            }       
        }     
    }

    println!("Initial state {:?}",lanternfish);

    for day in 1..number_days+1{
        let mut new_lanternfish: HashMap<u8, u64> = HashMap::new();

        for age in lanternfish.keys() {
            if *age == 0 {                            
                if new_lanternfish.get(&u8::from(6)) != None {                                    
                    *new_lanternfish.get_mut(&u8::from(6)).unwrap() += *lanternfish.get(age).unwrap();
                
                }else{
                    new_lanternfish.insert(6, *lanternfish.get(age).unwrap());
                }                 
                
                new_lanternfish.insert(8, *lanternfish.get(age).unwrap());                

            }else{                                           
                if new_lanternfish.get(&(age-1)) != None {
                    *new_lanternfish.get_mut(&(age-1)).unwrap() += *lanternfish.get(age).unwrap();
                }else{
                    new_lanternfish.insert(*age-1, *lanternfish.get(age).unwrap());
                }                             
            }
        }

        lanternfish = new_lanternfish;  
        println!("After {} days: {:?}",day,lanternfish);      
    }
    
    // Fish counter
    let mut fish_counter:u64 = 0;
    for age in lanternfish.keys() {
        fish_counter+= *lanternfish.get(&age).unwrap() as u64;
    }

    let result:String = format!("After {} there are {} fish",number_days,fish_counter);
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day6("./input/day6.txt".to_string(),256).unwrap();
    println!("{}", result);

    Ok(())
}



