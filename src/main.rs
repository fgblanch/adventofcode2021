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



#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
struct Point{
    row:u32,
    col:u32
}

fn get_min_point(points_to_visit: &Vec<Point>, distances:&HashMap<Point, u32>) -> u32 {
    let mut result:u32 = 0;
    let mut min_distance:u32 = u32::MAX;

    for (i,point) in points_to_visit.iter().enumerate(){
        let tmp_distance:u32 = *distances.get(&point).unwrap();
        if tmp_distance<min_distance{
            result = i as u32;
            min_distance= tmp_distance;
        }

    }
    result
}

const NODES_ARRAY_LEN: usize = 100; // test data

fn day15(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
    let mut risk_levels: [[u8; NODES_ARRAY_LEN*5]; NODES_ARRAY_LEN*5] = [[0;NODES_ARRAY_LEN*5];NODES_ARRAY_LEN*5]; 
    
    let mut points_to_visit:Vec<Point> = Vec::new();
    let mut distances:HashMap<Point, u32> = HashMap::new();
    let mut prevs:HashMap<Point, Point> = HashMap::new();
    

    let mut line_counter:u32 = 0;

    for line in buffered.lines() {        
        let current:String = line?; 
        for (i,elem) in current.chars().enumerate(){
            for j in 0..5{
                for k in 0..5{
                    let orig_val:u8 = String::from(elem).parse().unwrap();
                    let mut new_val:u16 = orig_val as u16 + j as u16 + k as u16;

                    if new_val > 9{
                        if new_val == 18{
                            new_val = 1;
                        }else{
                            new_val = new_val-9;
                        }
                    }

                    risk_levels
                    [(line_counter+(j*NODES_ARRAY_LEN as u32)) as usize]
                    [i+(k*NODES_ARRAY_LEN as u32) as usize] 
                    = new_val as u8;           
                }
            }
        }
        line_counter+=1;
    }

    for _i in 0..risk_levels.len(){
        for _j in 0..risk_levels.len(){
            let tmp_point = Point{ row: _i as u32, col: _j as u32};
            points_to_visit.push(tmp_point);
            distances.insert(tmp_point, u32::MAX);
            prevs.insert(tmp_point, Point{ row: 0, col:0});
        }
    }

    println!("{}", line_counter);
    println!("{:?}",risk_levels);
    //println!("Size: {} {}",risk_levels.len(),risk_levels[risk_levels.len()-1 as usize].len());

    let mut source:&mut u32 = distances.get_mut(&Point{ row: 0, col:0}).unwrap();
    *source = 0;
    
    let target = Point{col: ((NODES_ARRAY_LEN*5)-1) as u32 ,row: ((NODES_ARRAY_LEN*5)-1) as u32};

    let mut point_distance:u32 = 0;
    
    //println!("{:?}", distances);

    while !points_to_visit.is_empty(){
          let index:u32 = get_min_point(&points_to_visit, &distances);
          let point:Point = points_to_visit.remove(index as usize);
          point_distance = distances.get(&point).unwrap().clone();
          println!("Checking! {:?} {}", point, point_distance);

            // Check if point is the target.
            if point == target{
                println!("Encontrado! {:?} {}", point, point_distance);
                break;
            }else{
                // create temp neighbors vector 
                let mut neighbors:Vec<Point> = Vec::new();
                
                // right
                if point.col < ((NODES_ARRAY_LEN*5)-1) as u32{
                    neighbors.push(Point{ row: point.row, col: point.col+1});
                }

                // down
                if point.row < ((NODES_ARRAY_LEN*5)-1) as u32{
                    neighbors.push(Point{ row: point.row+1, col: point.col});
                }

                // left
                if point.col > 0 {
                    neighbors.push(Point{ row: point.row, col: point.col-1});
                }

                // up
                if point.row> 0 {
                    neighbors.push(Point{ row: point.row-1, col: point.col});
                }

                for neighbor in neighbors{
                    if points_to_visit.contains(&neighbor){
                        let new_distance:u32 = point_distance + (risk_levels[neighbor.row as usize][neighbor.col as usize] as u32);
                        let old_distance = distances.get_mut(&neighbor).unwrap();
                        if new_distance< *old_distance{
                            *old_distance = new_distance;
                            prevs.insert(neighbor, point);
                        }
                    }
                }
            }           
    }


    //println!("{:?}", distances);
    //println!("{:?}", prevs);


    let result:String = format!("{}",point_distance);
    
    Ok(result)
}


fn main() -> Result<(), Error> {
    let result:String = day15("./input/day15.txt".to_string()).unwrap(); //input data
    //let result:String = day15("./test/day15.txt".to_string()).unwrap(); // test data
    println!("result: {}", result);

    Ok(())
}



