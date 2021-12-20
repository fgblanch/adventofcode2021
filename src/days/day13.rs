use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq,Eq, Debug, Clone, Hash)]
struct Point{
    x:u32,
    y:u32
}

#[derive(PartialEq, Debug, Clone)]
struct Instruction{
    x_or_y:bool,
    line: u32
}

fn paint_board(points:&HashMap<Point,u32>){
    let mut x_max:u32 = 0;
    let mut y_max:u32 = 0;

    for point in points.keys(){
        x_max = x_max.max(point.x);
        y_max = y_max.max(point.y);        
    }

    for i in 0..y_max+1{
        for j in 0..x_max+1{
            let point = points.get(&Point{x: j, y:i});
            if point != None{
                print!("#");
            }else{
                print!(".");
            }
        }
        print!("\n");
    }
}


fn day13(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
    let mut folding_section:bool = false;
    let mut points_read:Vec<Point> = Vec::new();
    let mut instructions:Vec<Instruction> = Vec::new();

    for line in buffered.lines() {        
        let current:String = line?; 
        if current != ""{
            if !folding_section{
                // get dots positions
                let temp_vec:Vec<&str> = current.split(',').collect();
                let point:Point = Point{
                    x: temp_vec[0].parse().unwrap(),
                    y: temp_vec[1].parse().unwrap()
                };

                points_read.push(point);
            }else{
                // get folding instructions
                let re = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
                let caps = re.captures(&current).unwrap();
                let x_or_y;
                if caps.get(1).unwrap().as_str() == "x"{
                    x_or_y= false
                }else{
                    x_or_y= true
                }
                
                let ins:Instruction = Instruction{
                    x_or_y: x_or_y,
                    line: caps.get(2).unwrap().as_str().parse().unwrap()
                };
                instructions.push(ins);
            }
        }else{
            folding_section = true;
        }
    }

    println!("points: {:?}", points_read);
    println!("instructions: {:?}", instructions);


    let mut points:HashMap<Point,u32> = HashMap::new();

    for point in points_read{
        points.insert(point, 1);
    }

    for ins in instructions{
        let mut points_tmp:HashMap<Point,u32> = HashMap::new();
        
        for point in points.keys(){
            if ins.x_or_y{ // fold in y
                if point.y <ins.line{
                    points_tmp.insert(point.clone(), points.get(point).unwrap()+1);
                }else{
                    points_tmp.insert(Point{
                        x: point.x,
                        y: point.y-( (point.y-ins.line)*2)
                    }, points.get(point).unwrap()+1);
                }
            }else{ //fold in x
                if point.x <ins.line{
                    points_tmp.insert(point.clone(), points.get(point).unwrap()+1);
                }else{
                    points_tmp.insert(Point{
                        x: point.x-( (point.x-ins.line)*2),
                        y: point.y
                    }, points.get(point).unwrap()+1);
                }
            }
        }
        println!("\n Step x_or_y:{} line:{} result: {:?} len: {}",ins.x_or_y,ins.line,points_tmp,points_tmp.len());
        points = points_tmp;
    }

    println!("\n");
    paint_board(&points);
    println!("\n");

    let result:String = format!("{:?}",&points);
    
    Ok(result)
}
