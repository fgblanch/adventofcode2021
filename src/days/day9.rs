use std::fs::File;
use std::io::{BufReader, BufRead, Error};

#[derive(PartialEq, Debug, Clone)]
struct Point{
    row:u32,
    col:u32,
    val:u32
}

fn find_basin(heightmap:&Vec<Vec<u8>>, points_in_basin: &mut Vec<Point>, start_point:Point)-> Vec<Point>{
    let mut result:Vec<Point> = Vec::new();

    //if not in basin    
    if !points_in_basin.contains(&start_point) && start_point.val !=9{
        points_in_basin.push(start_point.clone());
        result.push(start_point.clone());

        // Left
        if start_point.col > 0 {
            let point_tmp = Point{
                row: start_point.row,
                col: start_point.col - 1,
                val: heightmap[start_point.row as usize][(start_point.col - 1) as usize] as u32
            };
            
            let mut basin_tmp:Vec<Point> = find_basin(heightmap,points_in_basin, point_tmp);
            result.append(&mut basin_tmp);
        }

        // Right 
        if start_point.col < (heightmap.get(start_point.row as usize).unwrap().len() - 1)  as u32 {
            let point_tmp = Point{
                row: start_point.row,
                col: start_point.col + 1,
                val: heightmap[start_point.row as usize][(start_point.col + 1) as usize] as u32
            };

            let mut basin_tmp:Vec<Point> = find_basin(heightmap,points_in_basin, point_tmp);
            result.append(&mut basin_tmp);
        }

        // Up
        if start_point.row > 0 {
            let point_tmp = Point{
                row: start_point.row -1,
                col: start_point.col,
                val: heightmap[(start_point.row -1) as usize][start_point.col as usize] as u32
            };
            
            let mut basin_tmp:Vec<Point> = find_basin(heightmap,points_in_basin, point_tmp);
            result.append(&mut basin_tmp);
        }

        // Down
        if start_point.row < (heightmap.len() - 1)  as u32 {
            let point_tmp = Point{
                row: start_point.row +1,
                col: start_point.col,
                val: heightmap[(start_point.row +1) as usize][start_point.col as usize] as u32
            };
            
            let mut basin_tmp:Vec<Point> = find_basin(heightmap,points_in_basin, point_tmp);
            result.append(&mut basin_tmp);
        }
    }
    result
}



fn day9(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    let mut heightmap:Vec<Vec<u8>> = Vec::new();
    let mut points_in_basin: Vec<Point> = Vec::new();
    let mut low_points:Vec<Point> = Vec::new();
    let mut basins:Vec<Vec<Point>> = Vec::new();

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
                low_points.push(Point{
                    row:i as u32,
                    col:j as u32,
                    val: current_val as u32
                })
            }
        }
        
    }

    println!("{:?}", low_points);

    for point in low_points{
        let basin = find_basin(&heightmap, &mut points_in_basin, point);
        basins.push(basin);
    }

    let mut basin_sizes:Vec<u32> = Vec::new();
    for basin in &basins{
        basin_sizes.push(basin.len() as u32);
        println!("basin size:{}",basin.len())
    }

    basin_sizes.sort();
    
    let result:String = format!("{:?}", basin_sizes);
    
    Ok(result)
}