use std::collections::HashMap;

fn day10(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    
    let mut part_b_scores:Vec<u64> = Vec::new();
    let mut wrong_closings:Vec<char> = Vec::new();
    let mut open_close: HashMap<char, char> = HashMap::new();
    open_close.insert('(', ')');
    open_close.insert('<', '>');
    open_close.insert('{', '}');
    open_close.insert('[', ']');

    let mut scores: HashMap<char, u32> = HashMap::new();
    scores.insert(')',3);
    scores.insert(']',57);
    scores.insert('}',1197);
    scores.insert('>',25137);

    let mut scores_partb: HashMap<char, u32> = HashMap::new();
    scores_partb.insert(')',1);
    scores_partb.insert(']',2);
    scores_partb.insert('}',3);
    scores_partb.insert('>',4);

    for line in buffered.lines() {                
        let current:String = line?;        
        let mut stack:Vec<char> = Vec::new();
        
        let mut is_corrupted:bool = false;
        for elem in current.chars(){
            if open_close.contains_key(&elem){
                stack.push(elem);
            }else{
                let last_elem:char = stack.pop().unwrap();
                if open_close.contains_key(&last_elem){
                    // compare last opening with found closing                    
                    if *open_close.get(&last_elem).unwrap() != elem {
                        println!("Wrong closing expected:{}  found:{}",last_elem,elem);
                        wrong_closings.push(elem);
                        is_corrupted = true;                        
                        break;
                    }
                }else{                    
                    panic!("There was an illegal character in the stack")
                }
            }
        }
        
        let mut missing_closings:Vec<char> = Vec::new();
        
        if !is_corrupted{            
            while stack.len() > 0 {            
                let last_elem:char = stack.pop().unwrap();
                
                if open_close.contains_key(&last_elem){
                    missing_closings.push(*open_close.get(&last_elem).unwrap());                
                }else{                    
                    panic!("There was an illegal character in the stack")
                }        
            }

            println!("Missing clossings{:?}",missing_closings);

            let mut score_b:u64 = 0;

            for missing in &missing_closings {
                score_b = (score_b*5) + (*scores_partb.get(&missing).unwrap() as u64);
            }
            println!("line score {}",score_b);
            part_b_scores.push(score_b);

        }
    }
    
    part_b_scores.sort();
    let final_partb_score = part_b_scores.get(part_b_scores.len()/2).unwrap();


    let mut score = 0;
    for closing in wrong_closings{
        score += scores.get(&closing).unwrap();
    }

    let result:String = format!("Part A:{} Part B: {}",score, final_partb_score);
    
    Ok(result)
}