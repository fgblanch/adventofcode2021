use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;
use std::collections::HashMap;

fn day5(input_path:String) -> Result<String, Error> {
    
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
   

    let mut vents: HashMap<String, u32> = HashMap::new();

    for line in buffered.lines() {        
        let current:String = line?; 
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        let caps = re.captures(&current).unwrap();
        
        let x1:u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let y1:u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let x2:u32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let y2:u32 = caps.get(4).unwrap().as_str().parse().unwrap();

        if x1 == x2 { // vertical
            println!("{} - Vertical", current);

            let y_min = y1.min(y2);
            let y_max = y1.max(y2);
            for i in y_min..y_max+1{
                let key:String = format!("{},{}",x1,i);                
                
                if vents.get(&key) != None {
                    *vents.get_mut(&key).unwrap() +=1;
                }else{
                    vents.insert(key, 1);
                }        

            }
        }else if y1 == y2 {
            println!("{} - Horizontal", current);

            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            for i in x_min..x_max+1{
                let key:String = format!("{},{}",i,y1);                
                
                if vents.get(&key) != None {
                    *vents.get_mut(&key).unwrap() +=1;
                }else{
                    vents.insert(key, 1);
                }                       
                
            }
        } else {                                              
            // check diagonal 45 degrees
            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);
            
            if true & ((x_max-x_min) == (y_max-y_min)) {
            
                println!("{} - Diagonal 45", current);

                let mut x_iter:u32 = x1;
                let mut y_iter:u32 = y1;

                let is_x_up = (x2 as i32 - x1 as i32)>=0; // up
                let is_y_up = (y2 as i32 - y1 as i32)>=0; // up

                for _i in 0..(x_max-x_min+1){
                    let key:String = format!("{},{}",x_iter,y_iter);                

                    if vents.get(&key) != None {
                        *vents.get_mut(&key).unwrap() +=1;
                    }else{
                        vents.insert(key, 1);
                    }                
                    if is_x_up{
                        x_iter+=1;
                    }else{
                        if x_iter >0{
                            x_iter-=1;
                        }
                    }

                    if is_y_up{
                        y_iter+=1;
                    }else{
                        if y_iter >0{
                            y_iter-=1;
                        }
                    }
                }

            }
            
        }
    }

    let mut vent_counter = 0;
    
    for key in vents.keys() {
        let val = *vents.get(key).unwrap();
        if val > 1 {
            vent_counter+=1;
        }
    }    

    //print_map(vents);

    let result:String = format!("Vent counter:{}",vent_counter);
    
    Ok(result)
}

fn print_map(vents: HashMap<String, u32>){
    
    let re = Regex::new(r"^(\d+),(\d+)$").unwrap();

    let mut x_min:u32 = u32::MAX;
    let mut x_max:u32 = u32::MIN;
    let mut y_min:u32 = u32::MAX;
    let mut y_max:u32 = u32::MIN;
            

    for key in vents.keys() {
        let caps = re.captures(&key).unwrap();
        let x:u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let y:u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        
        x_min = x.min(x_min);
        x_max = x.max(x_max);
        y_min = y.min(y_min);
        y_max = y.max(y_max);
    }

    println!("{},{},{},{}",x_min,x_max,y_min,y_max);

    for j in y_min..y_max+1{
        for i in x_min..x_max+1{        
            let key:String = format!("{},{}",i,j);                
            if vents.get(&key) != None {
                print!("{}", *vents.get(&key).unwrap());
            }else{
                print!(".")
            }         
        }
        print!("\n")
    }
}

fn main() -> Result<(), Error> {
    //day1_a("day1_input.txt".to_string())?;
    //day1_b("day1_b_test.txt".to_string())?;
    //let result:String = day1_b("./input/day1_input.txt".to_string()).unwrap(); // Answer: 1516
    //let result:String = day2("./input/day2_input.txt".to_string()).unwrap();

    //let result:String = day3_b("./input/day3_input.txt".to_string()).unwrap();
    //let result:String = day3_b("./tests/day3_b_test.txt".to_string()).unwrap();

    //let result:String = day4("./input/day4.txt".to_string(), false).unwrap();

    let result:String = day5("./input/day5.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



