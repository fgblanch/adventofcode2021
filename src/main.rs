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

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point{
    row:u32,
    col:u32
}

<<<<<<< HEAD
fn find_path(risk_levels: &[[u8; 100]; 100], point:&Point, cost:u32, min_cost_found:u32, path: &mut Vec<Point>) -> u32 {
    let mut result:u32 = min_cost_found;
    let mut new_min_cost = min_cost_found;

    println!("Entra {:?}, cost:{} min_cost:{}", point, cost,min_cost_found);
    if cost < min_cost_found{
        if (point.row == (risk_levels.len()-1) as u32) & (point.col==(risk_levels[risk_levels.len()-1 as usize].len()-1) as u32) {
            // we are at the end cell, add it to the path and return            
            
            println!("\n End cost:{} path: {:?}\n", cost, path);
            result = cost;
            println!("Sale {:?}, cost:{} result:{}", point, cost, result);
            
=======
    for line in buffered.lines() {        
        let current:String = line?; 
        
        if first_line{
            if current == "" {
                first_line = false;
            }else{
                input_comp=current;
            }
>>>>>>> 43f6a651c5ae7eab85902beea8e51af536f25fd5
        }else{
            // evaluate the possible directions and it not in path keep recursing
            let mut new_pos:Point = point.clone();        
            let mut new_cost:u32 = cost;
            let mut winner_point:Point = point.clone();        

            //println!("right? {}",point.col < (risk_levels[risk_levels.len()-1 as usize].len()-1) as u32);

            // right
            if point.col < (risk_levels[risk_levels.len()-1 as usize].len()-1) as u32{
                new_pos = Point{ row: point.row, col: point.col+1};
                    if !path.contains(&new_pos){
                    
                        path.push(new_pos.clone());
                        new_cost = find_path(risk_levels, &new_pos, cost + risk_levels[new_pos.row as usize][new_pos.col as usize] as u32,  new_min_cost, path);
                        
                        if new_cost < new_min_cost{
                            println!("Vuelve!");
                            result = new_cost;
                            winner_point = new_pos;
                            new_min_cost =result;
                        }

                        path.pop(); 
                    
                }
            }
            
            if point.row < (risk_levels.len()-1) as u32{       // down
                new_pos = Point{ row: point.row+1, col: point.col};
                if !path.contains(&new_pos){
                    
                        path.push(new_pos.clone());
                        new_cost = find_path(risk_levels, &new_pos, cost + risk_levels[new_pos.row as usize][new_pos.col as usize] as u32,new_min_cost, path);

                        if new_cost < new_min_cost{
                            println!("Vuelve! 2");
                            result = new_cost;
                            winner_point = new_pos;
                            new_min_cost =result;
                        }
                        println!("Sale! 3");
                        path.pop();
                    
                }
            }
            
            
            if point.col > 0 { // left
                new_pos = Point{ row: point.row, col: point.col-1};            
                if !path.contains(&new_pos){
                    
                        path.push(new_pos.clone());
                        new_cost = find_path(risk_levels, &new_pos, cost + risk_levels[new_pos.row as usize][new_pos.col as usize] as u32,new_min_cost, path);

                        if new_cost < new_min_cost{
                            result = new_cost;
                            winner_point = new_pos;
                            new_min_cost =result;
                        }

                        path.pop();
                    
                }
            }
            
            if point.row> 0 { // up
                new_pos = Point{ row: point.row-1, col: point.col};           
                if !path.contains(&new_pos){
                    
                        path.push(new_pos.clone());
                        new_cost = find_path(risk_levels, &new_pos, cost + risk_levels[new_pos.row as usize][new_pos.col as usize] as u32,new_min_cost, path);

                        if new_cost < new_min_cost{
                            result = new_cost;
                            winner_point = new_pos;
                            new_min_cost =result;
                        }

                        path.pop();
                    
                }
            }

            //path.push(winner_point.clone());
            println!("Sale {:?}, cost:{} result:{}", point, cost, result);
        }
    }
    result
}

<<<<<<< HEAD



fn day15(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
//    let mut risk_levels: [[u8; 10]; 10] = [[0;10];10]; // test data
    let mut risk_levels: [[u8; 100]; 100] = [[0;100];100];  // input  data

    let mut line_counter:u8 = 0;

    for line in buffered.lines() {        
        let current:String = line?; 
        for (i,elem) in current.chars().enumerate(){
            risk_levels[line_counter as usize][i] = String::from(elem).parse().unwrap();
        }
        line_counter+=1;
    }

    println!("{}", line_counter);
    println!("{:?}",risk_levels);
    println!("Size: {} {}",risk_levels.len(),risk_levels[risk_levels.len()-1 as usize].len());


    let new_pos = Point{ row: 0, col: 0};
    let mut path:Vec<Point> = Vec::new();
    path.push(new_pos.clone());
    let result = find_path(&risk_levels, &new_pos, 0,u32::MAX, &mut path);

    let result:String = format!("{}",result);
=======
    println!("input:{} expansions:{:?}",input_comp, expansions);

    let mut window:String = String::from("");
    let mut occurences:HashMap<String,u64> = HashMap::new();
    let mut occurences_chars:HashMap<String,u64> = HashMap::new();

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
    

    for i in 0..40{
        println!("Step: {}", i);
        println!("{:?}",occurences_chars);
        println!("{:?}",occurences);
        

        let mut new_occurences:HashMap<String,u64> = HashMap::new();

        for key in occurences.keys(){
           
            
            let new_polymer:&String = expansions.get(key).unwrap();                
            let new_key_a:String = format!("{}{}",key.chars().nth(0).unwrap().to_string(),new_polymer);
            let new_key_b:String = format!("{}{}",new_polymer,key.chars().nth(1).unwrap().to_string(),);

            let counter_a = new_occurences.entry(new_key_a.to_string()).or_default();
            *counter_a+=occurences.get(key).unwrap();

            let counter_b = new_occurences.entry(new_key_b.to_string()).or_default();
            *counter_b+=occurences.get(key).unwrap();        
            let counter_c = occurences_chars.entry(new_polymer.to_string()).or_default();
            *counter_c+=occurences.get(key).unwrap();
            
            
                           
        }

        occurences = new_occurences;     
    }
    
    println!("{:?}",occurences_chars);
    println!("{:?}",occurences);
    
    
        
    let result:String = format!("{:?}",occurences_chars);
>>>>>>> 43f6a651c5ae7eab85902beea8e51af536f25fd5
    
    Ok(result)
}


fn main() -> Result<(), Error> {
<<<<<<< HEAD
    let result:String = day15("./input/day15.txt".to_string()).unwrap(); //input data
    //let result:String = day15("./test/day15.txt".to_string()).unwrap(); // test data
=======
    let result:String = day14("./input/day14.txt".to_string()).unwrap();
>>>>>>> 43f6a651c5ae7eab85902beea8e51af536f25fd5
    println!("result: {}", result);

    Ok(())
}



