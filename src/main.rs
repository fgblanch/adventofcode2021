/*use std::io::Read;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get("https://adventofcode.com/2021/day/1/input")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
*/

use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;


fn day1_a(input_path:String) -> Result<(), Error> {

    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    let mut counter:u32 = 0;
    let mut prev_meassure:u32 = u32::MAX;

    for line in buffered.lines() {
        let current_meassure = line?.parse().unwrap();
        // println!("{}", current_meassure);
        if prev_meassure != u32::MAX && prev_meassure<current_meassure {
            counter = counter +1;    
        }
        prev_meassure = current_meassure;
    }

    println!("Day 1 Exercise A: {}", counter);
    
    Ok(())
} 

fn evaluate(current:u32, prev:u32) -> u32{
    let mut result = 0;
    if prev !=0 && prev < current {                
        result = 1;    
        print!(" increased!\n");
    }else if prev !=0 && prev > current {
        print!(" decreased \n");
    }else{
        print!("no change \n");
    }
    result
}

fn day1_b(input_path:String) -> Result<String, Error> {

    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    let mut counter:u32 = 0;
    let mut prev_meassure:u32 = 0;
    let mut current_meassure:u32 = 0;
    let mut window:Vec<u32> = Vec::new();     


    for line in buffered.lines() {
        let current_number = line?.parse().unwrap();
        
        if window.len()<3 {
            window.push(current_number);
        }else{
            print!(" window: {:?}", window);            
            current_meassure = window[0] + window[1] + window[2];
            print!("Current: {} Prev: {} ", current_meassure, prev_meassure);            
            window = vec![window[1], window[2],current_number];
            
            counter = counter + evaluate(current_meassure, prev_meassure);  
            
            prev_meassure = current_meassure;                    
        }                            
                            
    }

    if window.len()==3 {
        print!(" window: {:?}", window);            
        current_meassure = window[0] + window[1] + window[2];
        print!("Current: {} Prev: {} ", current_meassure, prev_meassure);
        
        counter = counter + evaluate(current_meassure, prev_meassure);
    }       
    let result:String = format!("{}", counter);
    println!("Day 1 Exercise B: {}", counter);

    Ok(result)
}

/* Day 2 */

#[derive(Debug, PartialEq)]
struct Submarine {
    pos: u32,  
    depth: u32,
    aim: u32
}



fn day2(input_path:String) -> Result<String, Error> {
    let result:String = "0".to_string();
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    let mut sub:Submarine = Submarine{pos: 0, depth:0, aim:0};

    let re = Regex::new(r"^(up|down|forward) (\d)$").unwrap();

    for line in buffered.lines() {
        let current:String = line?;
        let caps = re.captures(&current).unwrap();
        let command = caps.get(1).unwrap().as_str();
        let step:u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        match command {
            "up"      => {sub = Submarine{ pos:sub.pos, aim: sub.aim-step, depth: sub.depth}},
            "down"    => {sub = Submarine{ pos:sub.pos, aim: sub.aim+step, depth: sub.depth}},
            "forward" => {sub = Submarine{ pos:sub.pos+step, aim:sub.aim, depth: sub.depth + (sub.aim * step)}}
            _ => {println!("No");}
        }


        //println!("{}", step);
       
    }

    println!("{:?}", sub);
    println!("{}", sub.pos * sub.depth);


    Ok(result)
}

fn day3(input_path:String) -> Result<String, Error> {
    let mut result:String = String::from("");
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    let mut line_counter:u32 = 0;
    let mut one_counters:[u32; 12] = [0;12];


    for line in buffered.lines() {        
        let current:String = line?;
        
        for (i,c) in current.chars().enumerate() {
            if c == '1'{
                one_counters[i] = one_counters[i] + 1;
            }                        
        }
        
        line_counter = line_counter + 1;
    }

    let mut gamma:String = String::from("");
    let mut epsilon:String = String::from("");

    for (i, elem) in one_counters.iter().enumerate() {
        
            if *elem > line_counter/2 {
                gamma.push('1');
                epsilon.push('0')
            }else{
                gamma.push('0');
                epsilon.push('1')
            }
    }    

    //let bin_idx = "01110011001";
    let gamma_val = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_val = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", line_counter);
    println!("{:?}", one_counters);
    println!("gamma: {}, epsilon:  {}", gamma,epsilon);
    println!("gamma_val: {}, epsilon_val:  {} result: {} ", gamma_val,epsilon_val, gamma_val*epsilon_val);
    Ok(result)
}


fn main() -> Result<(), Error> {
    //day1_a("day1_input.txt".to_string())?;
    //day1_b("day1_b_test.txt".to_string())?;
    //let result:String = day1_b("./input/day1_input.txt".to_string()).unwrap(); // Answer: 1516
    //let result:String = day2("./input/day2_input.txt".to_string()).unwrap();

    let result:String = day3("./input/day3_input.txt".to_string()).unwrap();

    Ok(())
}



