use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashMap;
use std::iter::FromIterator;

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

fn sort_segments(word:&String) -> String{    
    let s_slice: &str = word;
    let mut chars: Vec<char> = s_slice.chars().collect();
    chars.sort();
    let s:String = String::from_iter(chars);
    s
}

fn day8(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
    let mut sum_output = 0;
    

    // variables to capture the known decodings.
    let mut two:String = String::new();  // one
    let mut four:String = String::new(); // four
    let mut three:String = String::new();
    let mut seven:String = String::new();
    let mut six_d:String = String::new();

    for line in buffered.lines() { 
        let current:String = line?;
        
        let mut encoder: HashMap<String, String> = HashMap::new();
        let mut decoder: HashMap<String, String> = HashMap::new();

        let temp_vec:Vec<&str> = current.split('|').collect();     

        let signal_patterns_string = String::from(temp_vec[0].trim());
        let digits_string = String::from(temp_vec[1].trim());

        println!("{}", signal_patterns_string);
        println!("{}", digits_string);

        let signals_temp:Vec<&str> = signal_patterns_string.split(' ').collect();     
        
        for signal in &signals_temp{
            if signal.trim().len() == 2 {
                two = String::from(signal.trim());
            }
            if signal.trim().len() == 3 {
                three = String::from(signal.trim());
            }
            if signal.trim().len() == 7{
                seven = String::from(signal.trim());
            }
            if signal.trim().len() == 4 {
                four = String::from(signal.trim());
            }
        }

        encoder.insert(String::from("1"),two.clone());
        encoder.insert(String::from("4"),four.clone());
        encoder.insert(String::from("7"),three.clone());
        encoder.insert(String::from("8"),seven.clone());

        for signal in &signals_temp{            
            if signal.trim().len() == 6 {                
                // this can be either number 0,6 or 9
                
                let mut has_all_three_segments = true;
                for segment in two.chars(){
                    has_all_three_segments = has_all_three_segments &  signal.contains(segment);
                }                                
                
                if has_all_three_segments{
                    // 6 or 9
                    let mut has_all_segments = true;
                    for segment in four.chars(){
                        has_all_segments = has_all_segments &  signal.contains(segment);
                    }                     
                    if has_all_segments{
                        // 9 = signal
                        encoder.insert(String::from("9"),String::from(signal.trim()));
                    }else{
                        // 0 = signal              
                        encoder.insert(String::from("0"),String::from(signal.trim()));
                    }
                }else{
                    // 6 = signal
                    encoder.insert(String::from("6"),String::from(signal.trim()));
                    six_d = String::from(signal.trim());
                }
            }
        }


        for signal in &signals_temp{
            if signal.trim().len() == 5 {                
                
                let mut segments_match_one = 0;
                for segment in two.chars(){
                    if signal.contains(segment) {
                        segments_match_one+=1;
                    }                    
                }
                
                if segments_match_one >1{ // it is number 3!
                    encoder.insert(String::from("3"),String::from(signal.trim()));
                }else{
                    let mut segments_match_six = 0;
                    for segment in six_d.chars(){
                        if signal.contains(segment) {
                            segments_match_six+=1;
                        }                    
                    }

                    if segments_match_six == 5{ // it is number 5!
                        encoder.insert(String::from("5"),String::from(signal.trim()));
                    }else{ // it is number 2"
                        encoder.insert(String::from("2"),String::from(signal.trim()));
                    }
                }                                
                
                // this can be either number 2,3 or 5
                /*if signal.contains(two.chars().nth(1).unwrap()){
                    if signal.contains(two.chars().nth(0).unwrap()){
                        // 3 = signal
                        
                    }else{
                        // 2 = signal
                        encoder.insert(String::from("2"),String::from(signal.trim()));
                    }
                }else{
                    // 5 = signal
                    encoder.insert(String::from("5"),String::from(signal.trim()));
                }*/
            }
        }


        println!("#2:{} #3:{} #4:{} #7:{}", two,three,four,seven);
        println!("{:?}", encoder);        

        for key in encoder.keys(){
            decoder.insert(sort_segments(&String::from(encoder.get(key).unwrap())), String::from(key));
        }
        println!("{:?}", decoder);     
        let digits_temp:Vec<&str> = digits_string.split(' ').collect();     
        let mut value_string:String = String::from("");

        for digit in &digits_temp{
            let str_to_decode:String = sort_segments(&String::from(digit.trim()));            
            let str_decoded:String = String::from(decoder.get(&str_to_decode).unwrap());//(sort_segments(decoder.get(digit.trim()).unwrap())).chars().nth(0).unwrap();
            println!("{} decodes to: {}", str_to_decode, str_decoded);     
            value_string+=&str_decoded;
        }
        let value_int:u32 = value_string.parse().unwrap();
        println!("{}", value_int); 
        sum_output +=value_int;
    }

    let result:String = format!("{}", sum_output);
    
    Ok(result)
}


fn main() -> Result<(), Error> {
    let result:String = day8("./input/day8.txt".to_string()).unwrap();
    println!("{}", result);

    Ok(())
}



