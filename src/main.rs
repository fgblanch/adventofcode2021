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
        //let temp_vec:Vec<&str> = current.split(',').collect();     
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

    for _i in 0..40{
        //println!("\n Input: {}",input_comp);
        window = String::from("");
        let mut new_input:String = String::from("");
        for c in input_comp.chars(){
            if window.len() < 2{
                window.push(c);
            }else{
                window = window.chars().nth(1).unwrap().to_string();
                window.push(c)
            }
            if window.len() == 2{
                let new_polymer:&String = expansions.get(&window).unwrap();                
                let mut tmp_str:String = format!("{}{}",window.chars().nth(0).unwrap().to_string(),new_polymer);                
                new_input.push_str(&tmp_str);                
            }

        }
        new_input.push_str(&window.chars().nth(1).unwrap().to_string());
        //println!("{}",new_input);
        input_comp= new_input;

    }

    let mut occurences:HashMap<String,u32> = HashMap::new();

    for c in input_comp.chars(){
        let counter = occurences.entry(String::from(c)).or_default();
        *counter+=1;
    }

    let result:String = format!("{:?}",occurences);
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day14("./input/day14.txt".to_string()).unwrap();
    println!("result: {}", result);

    Ok(())
}



