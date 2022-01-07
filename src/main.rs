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
// Returns a tuple with the sum and the last sequence index for the current package
fn sum_version_numbers(binary_sequence:&String, index:usize, state:ReadingState) -> (u32,usize) {
    let mut result:u32 = 0;
    let mut index_limit:usize = binary_sequence.len();
    
    match state {
        ReadingState::PacketStart => {
            println!("===========Packet start");
            let version:&str = &binary_sequence[index..(index+3)];
            let type_id:&str = &binary_sequence[(index+3)..(index+6)];
            println!("version: {}/{}  type: {}/{}",version,u8::from_str_radix(version, 2).unwrap(),
                                                       type_id,u8::from_str_radix(type_id, 2).unwrap());            
            
            let mut next_state = ReadingState::Operator;
            if type_id == "100"{ // Literal value
                next_state = ReadingState::LiteralValue;                
            }

            let result_tuple = sum_version_numbers(binary_sequence,index+6,next_state);

            result = u8::from_str_radix(version, 2).unwrap() as u32  + result_tuple.0;
            index_limit =  result_tuple.1;
        },            
        ReadingState::LiteralValue => {
            println!("===========Literal Value");   
            let mut found_last_group:bool = false;
            let mut index_offset:usize = 0; 
            let mut value:String = String::new();

            while !found_last_group{
                let group:&str = &binary_sequence[index+index_offset..(index+index_offset+5)];
                println!("group: {}",group); 
                value.push_str(&binary_sequence[index+index_offset+1..(index+index_offset+5)]);
                if &group[0..1] == "0"{
                    found_last_group = true;
                }
                index_offset+=5;
                
            }
            index_limit = index+ index_offset;

            println!("Literal value: {} length: {}",u64::from_str_radix(&value.to_string(), 2).unwrap(), index_limit);
        },
        ReadingState::Operator => {
            println!("===========Operator");            
            let length_type_id:&str = &binary_sequence[index..index+1];
            
            if length_type_id == "0"{ // total length: 15 bits 
                let bit_length:u32 = u32::from_str_radix(&binary_sequence[index+1..index+16], 2).unwrap();
                println!("bit length: {}",bit_length);
                
                let packets_start:usize = index+16;
                let mut new_index:usize = packets_start;

                while new_index < packets_start + bit_length as usize{
                    println!("new_index:{} limit:{}", new_index, packets_start + bit_length as usize);
                    let result_tuple = sum_version_numbers(binary_sequence,new_index,ReadingState::PacketStart);
                    println!("result_tuple:{:?} ", result_tuple);
                    result += result_tuple.0;
                    new_index = result_tuple.1;
                    println!("NEW new_index:{} ", new_index);
                }

                index_limit = new_index;

            }else if length_type_id == "1"{ // number of subpackets: 11 bits
                let packets_length = u32::from_str_radix(&binary_sequence[index+1..index+12], 2).unwrap();
                println!("# packets: {}",packets_length);

                let packets_start:usize = index+12;
                let mut new_index:usize = packets_start;

                for i in 0..packets_length{ // La longitud de los sub packets es dinámica
                    let result_tuple = sum_version_numbers(binary_sequence,new_index,ReadingState::PacketStart);
                    result += result_tuple.0;    
                    new_index = result_tuple.1;
                }

                index_limit = new_index;
            }
        }
    }
    println!("result: {}, index_limit:{}", result, index_limit);
    (result, index_limit)
}

// Returns a tuple with the sum and the last sequence index for the current package
fn calculate_value(binary_sequence:&String, index:usize) -> (u64,usize) {
    let mut result:u64 = 0;
    let mut index_limit:usize = binary_sequence.len();    
    
    println!("===========Packet start");
    let version:&str = &binary_sequence[index..(index+3)];
    let type_id:&str = &binary_sequence[(index+3)..(index+6)];
    println!("version: {}/{}  type: {}/{}",version,u8::from_str_radix(version, 2).unwrap(),
                                                type_id,u8::from_str_radix(type_id, 2).unwrap());            
    
    let  index_after_header = index+6;
        
    if type_id == "100"{ // literal value        
            println!("====================Literal Value");   
            let mut found_last_group:bool = false;
            let mut index_offset:usize = 0; 
            let mut value:String = String::new();

            while !found_last_group{
                let group:&str = &binary_sequence[index_after_header+index_offset..(index_after_header+index_offset+5)];
                println!("group: {}",group); 
                value.push_str(&binary_sequence[index_after_header+index_offset+1..(index_after_header+index_offset+5)]);
                if &group[0..1] == "0"{
                    found_last_group = true;
                }
                index_offset+=5;
                
            }
            index_limit = index_after_header+ index_offset;
            result = u64::from_str_radix(&value.to_string(), 2).unwrap();
            println!("Literal value: {} length: {}",result, index_limit);        
            
    }else{       
        println!("====================Operator");            
        let length_type_id:&str = &binary_sequence[index_after_header..index_after_header+1];        
        let mut operands:Vec<u64> = Vec::new();

        if length_type_id == "0"{ // total length: 15 bits 
            let bit_length:u32 = u32::from_str_radix(&binary_sequence[index_after_header+1..index_after_header+16], 2).unwrap();
            println!("bit length: {}",bit_length);
            
            let packets_start:usize = index_after_header+16;
            let mut new_index:usize = packets_start;            

            while new_index < packets_start + bit_length as usize{
                println!("new_index:{} limit:{}", new_index, packets_start + bit_length as usize);
                let result_tuple = calculate_value(binary_sequence,new_index);
                println!("result_tuple:{:?} ", result_tuple);                
                operands.push(result_tuple.0);
                new_index = result_tuple.1;
                println!("NEW new_index:{} ", new_index);
            }

            index_limit = new_index;

        }else if length_type_id == "1"{ // number of subpackets: 11 bits
           
            let packets_length = u32::from_str_radix(&binary_sequence[index_after_header+1..index_after_header+12], 2).unwrap();
            println!("# packets: {}",packets_length);

            let packets_start:usize = index_after_header+12;
            let mut new_index:usize = packets_start;

            // TODO: For all operands calculate result based on operation

            for i in 0..packets_length{ // La longitud de los sub packets es dinámica
                let result_tuple = calculate_value(binary_sequence,new_index);
                operands.push(result_tuple.0);
                new_index = result_tuple.1;
            }

            index_limit = new_index;
        }

        

        result = match type_id {
            "000" => {  // sum
                let mut result_value:u64 = 0;
                for op in operands{
                    result_value+= op;
                }
                result_value
            }, 
            "001" => { // product
                let mut result_value:u64 = 1;
                for op in operands {
                    result_value*= op;
                }
                result_value
            },                
            "010" => { // min
                let mut result_value:u64 = u64::MAX;                
                for op in operands{
                    result_value= result_value.min(op);
                }
                result_value
            },
            "011" => {  // max
                let mut result_value:u64 = u64::MIN;                
                for op in operands{
                    result_value= result_value.max(op);
                }
                result_value
            },
            "101" => {  // gt
                let mut result_value:u64 = 0;
                if operands.len() > 1{
                    if operands[0] > operands[1]{
                        result_value = 1;
                    }
                }
                result_value
            },
            "110" => { // lt
                let mut result_value:u64 = 0;
                if operands.len() > 1{
                    if operands[0] < operands[1]{
                        result_value = 1;
                    }
                }
                result_value
            },
            "111" => { // eq
                let mut result_value:u64 = 0;
                if operands.len() > 1{
                    if operands[0] == operands[1]{
                        result_value = 1;
                    }
                }
                result_value
            }, 
            &_ => { panic!("operation not supported")}
        }
    }
    
    println!("Operator result: {}, index_limit:{}", result, index_limit);
    (result, index_limit)
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
        println!("{} lenght: {}",binary_input, binary_input.len());
        //assert_eq!("11101110000000001101010000001100100000100011000001100000", binary_input);        
    }
    //let result_value = sum_version_numbers(&binary_input,0,ReadingState::PacketStart);
    let result_value = calculate_value(&binary_input, 0);
    let result:String = format!("{:?}", result_value);    
    Ok(result)
}


fn main() -> Result<(), Error> {
    let result:String = day16("./input/day16.txt".to_string()).unwrap(); //input data
    //let result:String = day16("./test/day16.txt".to_string()).unwrap(); // test data
    println!("result: {}", result);

    Ok(())
}



