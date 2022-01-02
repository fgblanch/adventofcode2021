use std::collections::VecDeque;

#[derive(PartialEq, Debug, Eq,)]
struct Instruction{
    ins: String,
    a:String,
    b:String
}

fn resolve_param(param:&String, registers:[i64;4]) -> i64 {
    let mut result:i64 = 0;
    if param == "w" {
        result = registers[0];
    }else if param == "x"{
        result = registers[1];
    }else if param == "y"{
        result = registers[2];
    }else if param == "z"{
        result = registers[3];
    }else{
        if !param.is_empty(){
            result = param.parse().unwrap();
        }
    }
    result
}

fn resolve_output(param:&String) -> usize{
    let mut result:usize = 0;    
    if param == "w" {
        result = 0;
    }else if param == "x"{
        result = 1;
    }else if param == "y"{
        result = 2;
    }else if param == "z"{
        result = 3;
    }
    result
}

#![recursion_limit = "10000000"]

fn search(instructions:&Vec<Instruction>, ops_i:usize, registers:&mut [i64;4])->(bool,String){
    
    println!("op: {} remaining: {} Reg:{:?}",ops_i,instructions.len(), registers);

    if ops_i >= instructions.len(){
        return (false, String::from(""))
    }

    if registers[3]  >= 10000000{
        return (false, String::from("Not found"))
    }

    /*if (registers[3] == 0) & (ops_i>20){
        return (true, String::from(""));
    }*/

    println!("op: {} Reg:{:?}",ops_i, registers);

    let to_execute:&Instruction = instructions.get(ops_i).unwrap();

    let arg_a:i64 = resolve_param(&to_execute.a,*registers);
    let arg_b:i64 = resolve_param(&to_execute.b,*registers);
    let out_i:usize = resolve_output(&to_execute.a);

    if to_execute.ins == "inp"{
        // search through params recursively
        for i in (1..10).rev(){
            registers[out_i] = i as i64;
            let result:(bool,String) = search(instructions,ops_i +1, registers);
            if result.0{
                return (true, format!("{}{}",result.1,i));
            }
        }
        return (false, String::from("Not found"))
    }else if to_execute.ins == "add"{
        registers[out_i] = arg_a + arg_b;
    }else if to_execute.ins == "mul"{
        registers[out_i] = arg_a * arg_b;
    }else if to_execute.ins == "div"{
        registers[out_i] = arg_a / arg_b;
    }else if to_execute.ins == "mod"{
        registers[out_i] = arg_a % arg_b;
    }else if to_execute.ins == "eql"{
        if  arg_a ==  arg_b{
            registers[out_i] = 1;
        }else{
            registers[out_i] = 0;
        }            
    }else{
        panic!("Operation not supported in ALU")
    }        
    
    return search(instructions,ops_i +1, registers);
    
    //println!("Intruction: {:?} Registers output:{:?}",to_execute,registers);
    /*if registers[3] > 100000000 {
        break;
    }*/
}


fn day24(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut instructions:Vec<Instruction> = Vec::new();

    
    for line in buffered.lines() {        
        let current:String = line?; 
        let temp_vec:Vec<&str> = current.split(' ').collect();
        let mut ins_temp = Instruction{
            ins: String::from(temp_vec[0]),
            a: String::from(temp_vec[1]),
            b: String::from("")
        };
        if temp_vec.len() > 2{
            ins_temp.b = String::from(temp_vec[2]);     
        }
        instructions.push(ins_temp);
    }    

    // recursive try
    let mut registers:[i64;4] = [0;4]; // Registers: w, x, y, z    
    let result:(bool,String) = search(&instructions, 0, &mut registers);


    // find 14 digit number: e.g 13579246899999
    //let mut input:VecDeque<i64> = VecDeque::new();
    
    
    // bruteforce solution 99999974968947
    /*for i in (11111111111111 as u64..99999999999999 as u64).rev(){
                
            result_number = format!("{}",i);
            
        if !result_number.contains('0'){ // don't test numbers that contain a 0.
            if String::from(result_number.chars().nth(12).unwrap())=="1"{

                for c in result_number.chars(){
                    input.push_back(String::from(c).parse().unwrap());
                }
                        
                let mut registers:[i64;4] = [0;4]; // Registers: w, x, y, z

                for to_execute in &instructions{            
                    let arg_a:i64 = resolve_param(&to_execute.a,registers);
                    let arg_b:i64 = resolve_param(&to_execute.b,registers);
                    let out_i:usize = resolve_output(&to_execute.a);

                    if to_execute.ins == "inp"{
                        registers[out_i] = input.pop_front().unwrap();
                    }else if to_execute.ins == "add"{
                        registers[out_i] = arg_a + arg_b;
                    }else if to_execute.ins == "mul"{
                        registers[out_i] = arg_a * arg_b;
                    }else if to_execute.ins == "div"{
                        registers[out_i] = arg_a / arg_b;
                    }else if to_execute.ins == "mod"{
                        registers[out_i] = arg_a % arg_b;
                    }else if to_execute.ins == "eql"{
                        if  arg_a ==  arg_b{
                            registers[out_i] = 1;
                        }else{
                            registers[out_i] = 0;
                        }            
                    }else{
                        panic!("Operation not supported in ALU")
                    }        
                    //println!("Intruction: {:?} Registers output:{:?}",to_execute,registers);
                    if registers[3] > 100000000 {
                        break;
                    }
                
                }

                
                // find 14 digit number: e.g 13579246899999
                println!("Tested: {} Reg:{:?}",result_number, registers);
                

                if registers[3] == 0 {
                    println!("Valid number found: {}", result_number);
                    break;
                }
            }
        }
    }*/

    let result:String = format!("{}",result.1);
    
    Ok(result)
}
