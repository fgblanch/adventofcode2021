use std::fs::File;
use std::io::{BufReader, BufRead, Error};
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

fn day10(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
    let mut stack:Vec<char> = Vec::new();
    let mut wrong_closings:Vec<char> = Vec::new();
    let mut open_close: HashMap<char, char> = HashMap::new();
    open_close.insert('(', ')');
    open_close.insert('<', '>');
    open_close.insert('{', '}');
    open_close.insert('[', ']');

    let mut scores: HashMap<char, u32> = HashMap::new();
    scores.insert(')',3);
    scores.insert(']',57);
    scores.insert('}',1197);
    scores.insert('>',25137);

    for line in buffered.lines() {        
        let current:String = line?; 
        
        for elem in current.chars(){
            if open_close.contains_key(&elem){
                stack.push(elem);
            }else{
                let last_elem:char = stack.pop().unwrap();
                if open_close.contains_key(&last_elem){
                    // compare last opening with found closing                    
                    if *open_close.get(&last_elem).unwrap() != elem {
                        println!("Wrong closing expected:{}  found:{}",last_elem,elem);
                        wrong_closings.push(elem);
                        break;
                    }
                }else{
                    // error había un ciere en la cola
                    panic!("There was an illegal character in the queue")
                }
            }// if elem is one of the openings
            //         push opening to stack
            // else
        }
    }
    println!("{:?}",wrong_closings);

    let mut score = 0;
    for closing in wrong_closings{
        score += scores.get(&closing).unwrap();
    }

    let result:String = format!("{}",score);
    
    Ok(result)
}


fn main() -> Result<(), Error> {
    let result:String = day10("./input/day10.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



