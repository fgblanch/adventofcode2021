use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashMap;

fn count_lowecase( current_path: &mut Vec<String>) -> u32 {
    let mut counters:HashMap<String,u32> = HashMap::new();
    let mut max_counter:u32 = 0;
    for elem in current_path{
        if elem.chars().nth(0).unwrap().is_lowercase() & (elem != "start") & (elem != "end") {
            let counter = counters.entry(elem.to_string()).or_insert(0);
            *counter+=1; 
            max_counter=max_counter.max(*counter);
        }
    }

    max_counter
}


fn find_paths(connections:&HashMap<String,Vec<String>>, current_path: &mut Vec<String>, current_node:String) -> Vec<Vec<String>>{
    let mut result:Vec<Vec<String>> = Vec::new();

    let current_conns:&Vec<String> = connections.get(&current_node).unwrap();
    current_path.push(current_node.clone());
    
    for cave in current_conns{
        println!("testing {:?} + {}?  in path?: {} lowercase?: {} ",
                    current_path, 
                    cave, 
                    current_path.contains(cave),cave.chars().nth(0).unwrap().is_lowercase(),
                   
                );        

        if (cave != "start") & 
           (cave != "end") & 
           (!cave.chars().nth(0).unwrap().is_lowercase()  | 
              (cave.chars().nth(0).unwrap().is_lowercase() & (!current_path.contains(cave) | (current_path.contains(cave) & (count_lowecase(current_path)<2)) ) )){           
           println!("Entra 1 - sigue buscando");

           let mut temp_vec: Vec<Vec<String>> = find_paths(connections, current_path, String::from(cave));
           result.append(&mut temp_vec);
        }else{            
            if cave != "start" &&  (cave.chars().nth(0).unwrap().is_lowercase() & !current_path.contains(cave)){                
                current_path.push(String::from(cave));
                println!("Entra 2 - suma cadena: {:?}",current_path);            
                result.push(current_path.to_vec()); 
                current_path.pop();


                println!("y echa patras: {:?}",current_path);            
            }else{
                
                println!("Fin Mal camino");
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
    let result: Vec<Vec<String>> =find_paths(&connections, &mut current_path, String::from("start"));

    for comb in &result{
        println!("{:?}", comb);
    }


    let result:String = format!("{:?}",result.len());
    
    Ok(result)
}
