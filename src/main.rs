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

fn find_paths(connections:&HashMap<String,Vec<String>>, current_path: &mut Vec<String>, current_node:String ) -> Vec<String>{
    let mut result:Vec<String> = Vec::new();

    let current_conns:&Vec<String> = connections.get(&current_node).unwrap();
    current_path.push(current_node.clone());

    for cave in current_conns{
        println!("{:?}",current_path);
        println!("{}",cave);
        if (cave != "start") & (cave != "end") & 
            ( !cave.chars().nth(0).unwrap().is_lowercase()  | 
              (cave.chars().nth(0).unwrap().is_lowercase() & !current_path.contains(cave))){           
           let mut temp_vec: Vec<String> = find_paths(connections, current_path, String::from(cave));
           result.append(&mut temp_vec);
        }else{
            if cave != "start"{
                current_path.push(String::from(cave));
                result.append(&mut current_path.to_vec());
            }
        }
    }
    current_path.pop();
    result
}


fn day12(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut connections:HashMap<String,Vec<String>> = HashMap::new();
    
    for line in buffered.lines() {        
        let current:String = line?; 
        let temp_vec:Vec<&str> = current.split('-').collect();   
        
        connections.entry(String::from(temp_vec[0]))
            .or_default()
            .push(String::from(temp_vec[1]));

        connections.entry(String::from(temp_vec[1]))
            .or_default()
            .push(String::from(temp_vec[0]));             
    }

    println!("{:?}",connections);

    let mut  current_path = Vec::new();
    let result: Vec<String> =find_paths(&connections, &mut current_path, String::from("start"));


    let result:String = format!("{:?}",result);
    
    Ok(result)
}

fn main() -> Result<(), Error> {
    let result:String = day12("./test/day12_a.txt".to_string()).unwrap();
    println!("result: {}", result);

    Ok(())
}



