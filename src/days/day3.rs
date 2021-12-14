use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn day3(input_path:String) -> Result<String, Error> {
    let result:String = String::from("");
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    let mut line_counter:u32 = 0;
    let mut one_counters:[u32; 12] = [0;12];


    for line in buffered.lines() {        
        let current:String = line?;
        
        for (i,c) in current.chars().enumerate() {
            if c == '1'{
                one_counters[i] = one_counters[i] + 1;
            }                        
        }
        
        line_counter = line_counter + 1;
    }

    let mut gamma:String = String::from("");
    let mut epsilon:String = String::from("");

    for (i, elem) in one_counters.iter().enumerate() {
        
            if *elem > line_counter/2 {
                gamma.push('1');
                epsilon.push('0')
            }else{
                gamma.push('0');
                epsilon.push('1')
            }
    }    

    //let bin_idx = "01110011001";
    let gamma_val = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_val = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", line_counter);
    println!("{:?}", one_counters);
    println!("gamma: {}, epsilon:  {}", gamma,epsilon);
    println!("gamma_val: {}, epsilon_val:  {} result: {} ", gamma_val,epsilon_val, gamma_val*epsilon_val);
    Ok(result)
}

fn findRating(numbers:Vec<String>, most_common:bool, bit_number:u32) -> Result<String, Error> {
    let mut result = String::new();
    let numbers_len = numbers.len() as u32;

    //println!("bit number: {} numbers {:?}",bit_number, numbers);

    if numbers_len == 0{
        panic!("Empty vector!")
    }
    if numbers_len == 1{
        result = (*numbers.get(0).unwrap()).clone();    
    }else{
        // for each one of the numbers in the vector check if bit_number is equal to the most common
        // store them in a results vector
        // call the find rating function recursively with result_vector as parameter, most_common and bit_number +1
        // if bit_number+1 is > number.length throw an error (!not found)
        let mut counter:u32 = 0;
        let mut ones:Vec<String> = Vec::new();
        let mut zeroes:Vec<String> = Vec::new();
        
        for number in numbers{
            if bit_number>=(number.len()as u32){
                panic!("Empty vector! {}", bit_number)
            }
            if number.chars().nth(bit_number as usize).unwrap() == '1'{
                counter = counter + 1;
                ones.push(number.clone());
            }else{
                zeroes.push(number.clone());
            }
        }
        
        let next_bit_number:u32 =  bit_number+1;
        let counter_zeroes:u32 = numbers_len - counter;
        //println!("ones: {} zeroes {}", counter, counter_zeroes);
                
        if counter>=counter_zeroes{ // 1 is most common
            if most_common{ // are we looking for the most or least common?
                result = findRating(ones, most_common,next_bit_number)?;
            }else{
                result = findRating(zeroes, most_common, next_bit_number)?;
            }
        }else{ // 0 is most common
            if most_common{ // are we looking for the most or least common?
                result = findRating(zeroes, most_common, next_bit_number)?;
            }else{
                result = findRating(ones, most_common, next_bit_number)?;
            }
        }                
    }

    Ok(result)
}

fn day3_b(input_path:String) -> Result<String, Error> {
    
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut numbers:Vec<String> = Vec::new();

    for line in buffered.lines() {        
        let current:String = line?;
        numbers.push(current);            
    }

    let numbers_oxygen:Vec<String> = numbers.clone();
    let numbers_co2:Vec<String> = numbers.clone();

    let oxygen_rating = findRating(numbers_oxygen, true,0);
    let co2_rating = findRating(numbers_co2, false,0);

    let oxygen_rating_val = isize::from_str_radix(&oxygen_rating.unwrap(), 2).unwrap();
    let co2_rating_val = isize::from_str_radix(&co2_rating.unwrap(), 2).unwrap();

    println!("oxygen: {}, co2: {}",oxygen_rating_val,co2_rating_val);
    println!("result: {}",oxygen_rating_val*co2_rating_val);
    let result:String = format!("{}",oxygen_rating_val*co2_rating_val);

    Ok(result)
}