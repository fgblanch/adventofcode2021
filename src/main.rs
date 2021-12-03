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

fn main() -> Result<(), Error> {
    let path = "day1_input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut counter:u32 = 0;
    let mut prev_meassure:u32 = u32::MAX;

    for line in buffered.lines() {
        let current_meassure = line.unwrap().parse().unwrap();
        println!("{}", current_meassure);
        if(prev_meassure != u32::MAX && prev_meassure<current_meassure){
            counter = counter +1;    
        }
        prev_meassure = current_meassure;
    }

    println!("{}", counter);

    Ok(())
}



