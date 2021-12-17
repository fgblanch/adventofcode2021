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

fn day11(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
    let mut octopuses: [[u8; 10]; 10] = [[0;10];10];

    let mut line_counter:u8 = 0;
    for line in buffered.lines() {        
        let current:String = line?; 
        for (i,elem) in current.chars().enumerate(){
            octopuses[line_counter as usize][i] = String::from(elem).parse().unwrap();
        }
        line_counter+=1;
    }

    for _i in 0..100{ // steps

        let mut flashed:bool = false;
        let mut octopuses_temp: [[u8; 10]; 10] = [[0;10];10];

        for j in 0..10{
            for k in 0..10{
                if oct1
                octopuses_temp[j][k] = octopuses[j][k] +1


            } 
        }

    }
    
    
    
    
    
    
    
    
    
    println!("{:?}",octopuses);

    let result:String = format!("result");
    





    Ok(result)
}


fn main() -> Result<(), Error> {
    let result:String = day11("./test/day11.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



