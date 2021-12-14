/* Day 2 */
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;

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
