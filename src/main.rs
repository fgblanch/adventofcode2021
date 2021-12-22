use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;
use std::collections::HashMap;


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

fn day14(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
    let re:Regex = Regex::new(r"^([A-Z]+) -> ([A-Z]+)$").unwrap();
    let mut expansions:HashMap<String,String> = HashMap::new();
    let mut input_comp:String = String::new();
    let mut first_line:bool = true;

    for line in buffered.lines() {        
        let current:String = line?; 
        
        if first_line{
            if current == "" {
                first_line = false;
            }else{
                input_comp=current;
            }
        }else{
            let caps = re.captures(&current).unwrap();
            expansions.insert(String::from(caps.get(1).unwrap().as_str()),
                              String::from(caps.get(2).unwrap().as_str()));
        }
    }

    println!("input:{} expansions:{:?}",input_comp, expansions);

    let mut window:String = String::from("");
    let mut occurences:HashMap<String,u32> = HashMap::new();
    let mut occurences_chars:HashMap<String,u32> = HashMap::new();

    for c in input_comp.chars(){
        if window.len() < 2{
            window.push(c);
        }else{
            window = window.chars().nth(1).unwrap().to_string();
            window.push(c)
        }
        if window.len() == 2{
            let counter = occurences.entry(window.to_string()).or_default();
            *counter+=1;
        }

        let counter = occurences_chars.entry(String::from(c)).or_default();
        *counter+=1;


    }
    println!("{:?}",occurences);
    println!("{:?}",occurences_chars);

    for i in 0..10{
        println!("Step: {}", i);
        let mut new_occurences:HashMap<String,u32> = HashMap::new();

        for key in occurences.keys(){
            
            let new_polymer:&String = expansions.get(key).unwrap();                
            let new_key_a:String = format!("{}{}",key.chars().nth(0).unwrap().to_string(),new_polymer);
            let new_key_b:String = format!("{}{}",new_polymer,key.chars().nth(1).unwrap().to_string(),);

            let counter_a = new_occurences.entry(new_key_a.to_string()).or_default();
            *counter_a+=1;

            let counter_b = new_occurences.entry(new_key_b.to_string()).or_default();
            *counter_b+=1;        
            
            let counter_c = occurences_chars.entry(new_polymer.to_string()).or_default();
            *counter_c+=occurences.get(key).unwrap();
                           
        }

        occurences = new_occurences;
         println!("{:?}",occurences);
        println!("{:?}",occurences_chars);
    }

    println!("{:?}",occurences);
    println!("{:?}",occurences_chars);
    
        
    let result:String = format!("{:?}",occurences_chars);
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day14("./test/day14.txt".to_string()).unwrap();
    println!("result: {}", result);

    Ok(())
}



