use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::VecDeque;


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

#[derive(PartialEq, Debug, Eq,)]
struct Instruction{
    ins: String,
    a:String,
    b:String
}

fn day24(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut instructions:VecDeque<Instruction> = VecDeque::new();

    
    for line in buffered.lines() {        
        let current:String = line?; 
        let temp_vec:Vec<&str> = current.split(' ').collect();
        let mut ins_temp = Instruction{
            ins: String::from(temp_vec[1]),
            a: String::from(temp_vec[2]),
            b: String::from(temp_vec[3])
        };
        
        instructions.push_back(ins_temp);
    }

    let mut w:i32 = 0;
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut z:i32 = 0;


    while !instructions.is_empty(){
        let to_execute:Instruction = instructions.pop_front().unwrap();
        let mut arg_a:i32;
        let mut arg_b:i32;
        
        // todo: check the argument type and replace by value on arg_x

        if to_execute.ins == "inp"{

        }else if to_execute.ins == "add"{

        }else if to_execute.ins == "mul"{


        }else if to_execute.ins == "div"{

        }else if to_execute.ins == "mod"{

        }else if to_execute.ins == "eql"{

        }else{
            panic!("Operation not supported in ALU")
        }
        
    }


    // find 14 digit number: e.g 13579246899999



    let result:String = format!("result");
    
    Ok(result)
}


fn main() -> Result<(), Error> {
    //let result:String = day24("./input/day24.txt".to_string()).unwrap(); //input data
    let result:String = day24("./test/day24_c.txt".to_string()).unwrap(); // test data
    println!("result: {}", result);

    Ok(())
}



