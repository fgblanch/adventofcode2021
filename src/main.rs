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

 enum ReadingState {    
    PacketStart,
    LiteralValue,
    Operator
}


fn sum_version_numbers(binary_sequence:&String, index:usize, state:ReadingState) -> u32{
    let mut result:u32 = 0;
    
    match state {
        ReadingState::PacketStart => {
            let version:&str = &binary_sequence[index..(index+3)];
            let type_id:&str = &binary_sequence[(index+3)..(index+6)];
            println!("version: {}/{}  type: {}/{}",version,u8::from_str_radix(version, 2).unwrap(),
                                                       type_id,u8::from_str_radix(type_id, 2).unwrap());            
            if type_id == "100"{ // Literal value
                result = u8::from_str_radix(version, 2).unwrap() as u32  + sum_version_numbers(binary_sequence,index+6,ReadingState::LiteralValue);
            }else{ // operator

            }
        },            
        ReadingState::LiteralValue => {
            let mut found_last_group:bool = false;
            let mut index_offset:usize = 0; 
            let mut value:String = String::new();

            while !found_last_group{
                let group:&str = &binary_sequence[index+index_offset..(index+index_offset+5)];
                println!("{}",group);
                value.push_str(&binary_sequence[index+index_offset+1..(index+index_offset+5)]);
                if &group[0..1] == "0"{
                    found_last_group = true;
                }else{
                    index_offset+=5;
                }
            }
            println!("{}",u32::from_str_radix(&value.to_string(), 2).unwrap());
        },
        ReadingState::Operator => {
            // para cada uno de los subpaquetes hacer llamada recursiva al paquete con estado inicial e indice
        }
    }

    result
}



fn day16(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut binary_input:String = String::new();

    for line in buffered.lines() {        
        let current:String = line?;         
        
        for c in current.chars(){
            match c {
                '0' =>&binary_input.push_str("0000"),
                '1' =>&binary_input.push_str("0001"),
                '2' =>&binary_input.push_str("0010"),
                '3' =>&binary_input.push_str("0011"),
                '4' =>&binary_input.push_str("0100"),
                '5' =>&binary_input.push_str("0101"),
                '6' =>&binary_input.push_str("0110"),
                '7' =>&binary_input.push_str("0111"),
                '8' =>&binary_input.push_str("1000"),
                '9' =>&binary_input.push_str("1001"),
                'A' =>&binary_input.push_str("1010"),
                'B' =>&binary_input.push_str("1011"),
                'C' =>&binary_input.push_str("1100"),
                'D' =>&binary_input.push_str("1101"),
                'E' =>&binary_input.push_str("1110"),
                'F' =>&binary_input.push_str("1111"),
                _   =>&()
            };
        }
        println!("{}",binary_input);
        //assert_eq!("11101110000000001101010000001100100000100011000001100000", binary_input);
        sum_version_numbers(&binary_input,0,ReadingState::PacketStart);

    }

    let result:String = format!("result");
    
    Ok(result)
}


fn main() -> Result<(), Error> {
    //let result:String = day16("./input/day16.txt".to_string()).unwrap(); //input data
    let result:String = day16("./test/day16.txt".to_string()).unwrap(); // test data
    println!("result: {}", result);

    Ok(())
}



