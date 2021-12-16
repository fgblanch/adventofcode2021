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

fn day9(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    let mut heightmap:Vec<Vec<u8>> = Vec::new();
    let mut sum_low_points:u32 = 0;
    
    for line in buffered.lines() {        
        let current:String = line?; 
        let mut row:Vec<u8> = Vec::new();

        for a in current.chars(){
            print!("{}", a);
            row.push(String::from(a).parse().unwrap());
        }
        heightmap.push(row);
        print!("\n");
    }    

    for i in 0..heightmap.len(){
        let current_row:&Vec<u8> = heightmap.get(i).unwrap();
        let mut previous_row:&Vec<u8> = &Vec::new(); 
        let mut next_row:&Vec<u8> = &Vec::new(); 

        if i > 0{
            previous_row = heightmap.get(i-1).unwrap()
        }

        if i< heightmap.len()-1{
            next_row = heightmap.get(i+1).unwrap()
        }

        for j in 0..current_row.len(){            
            let mut is_lowest = true;
            let current_val:u8 = *current_row.get(j).unwrap();
            

            if j>0{ // compare it with left
                is_lowest = is_lowest & (current_val<*current_row.get(j-1).unwrap());
            }

            if j<current_row.len()-1{ //compare it with right
                is_lowest = is_lowest & (current_val<*current_row.get(j+1).unwrap());                
            }

            if *(&previous_row.len()) > (0 as usize){
                is_lowest = is_lowest & (current_val<*previous_row.get(j).unwrap());
            }
            
            if *(&next_row.len()) > (0 as usize){
                is_lowest = is_lowest & (current_val<*next_row.get(j).unwrap());
            }

            if is_lowest {
                println!("low point row:{} col:{} val:{}", i, j, current_val);
                sum_low_points += (current_val as u32) + 1;
            }
        }
        
    }

    let result:String = format!("{}", sum_low_points);
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day9("./input/day9.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



