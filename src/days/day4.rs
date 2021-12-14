#[derive(PartialEq, Debug)]
struct  BingoCard{
    size: u32,
    board: Vec<BingoCell>
}

impl BingoCard {
    fn new() -> BingoCard {
        BingoCard{
            size: 5,
            board: Vec::new()
        }
    }

    fn has_winner(&self) -> bool {
        let mut has_winner = false;
        if self.board.len() as u32 == (self.size*self.size){            
            
            for i  in 0..self.size{
                let mut row_winner = true;
                let mut col_winner = true;
                for j  in 0..self.size{
                    row_winner= row_winner & self.board[(i*self.size + j) as usize].checked;
                    col_winner= col_winner & self.board[(i+j*self.size) as usize].checked;
                }
                has_winner = has_winner | row_winner | col_winner
            }            

        }
        has_winner 
    }

    fn mark_number(&mut self, number:String){
        for cell in self.board.iter_mut(){
            if cell.value == number{
                cell.mark()
            }
        }
        ()
    }

    fn add_number(&mut self, number:String){
        if (self.board.len() as u32) < (self.size *  self.size){
            let cell:BingoCell = BingoCell{
                value: number,
                checked: false
            };
            self.board.push(cell);
            ()
        }else{
            panic!("Adding more numbers to board than its size.")
        }
    }

    fn sum_not_marked(&self) -> u32 {
        let mut sum:u32 = 0;
        for number in self.board.iter(){
            if !number.checked{
                let temp_parse:u32 = number.value.parse().unwrap();
                sum = sum + temp_parse;
            }
        }
        sum
    }
}

impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result:String = String::from("");
        let mut i:u32 = 0;
        for cell in self.board.iter(){
            if cell.checked{
                result = result + "(" + &cell.value + ")\t"
            }else{
                result = result + &cell.value + "\t"
            }
            if i % self.size == (self.size-1) {
                result = result + "\n"
            }

            i+=1;
        }
        result = result + "\n";
        write!(f, "{} ", result)
    }
}

#[derive(PartialEq, Debug)]
struct  BingoCell{
    value: String,
    checked: bool
}

impl BingoCell {
    fn mark(&mut self){
        self.checked = true
    }    
}

fn day4(input_path:String, a_or_b:bool) -> Result<String, Error> { // true for part a and false for b
    
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);    

    let mut line_counter:u32 = 0;
    let mut drawn_numbers:Vec<String> = Vec::new();
    let mut boards:Vec<BingoCard> = Vec::new();

    let mut current_board:BingoCard = BingoCard::new();
    for line in buffered.lines() {    
        let temp_line = line.unwrap();        

        if line_counter == 0 { // retrieving random numbers
            let temp_vec:Vec<&str> = temp_line.split(',').collect(); 
            for number in temp_vec {
                drawn_numbers.push(String::from(number));
            }
        }else{
            if (temp_line.len() as u32) > 0u32 {
                let temp_vec:Vec<&str> = temp_line.split(' ').collect(); 
                for number in temp_vec {
                    if number.len() > 0{               
                        current_board.add_number(String::from(number))
                    }
                }
            }else{
                // check if last baord is valid and add it to the vector of boards                
                if current_board.board.len() > 0{
                    boards.push(current_board);
                    // create a new temp board                 
                    current_board = BingoCard::new();
                }
            }
        }

        line_counter = line_counter + 1;
    }

    if current_board.board.len() > 0{ // We push the last board
        boards.push(current_board);
    }

    if a_or_b{
        let mut we_have_a_winner = false;

        for number in drawn_numbers{
            println!("Number: {}", number);
            let number_to_mark = &number;

            for board in boards.iter_mut(){
                (*board).mark_number(String::from(number_to_mark));
                println!("{}",board);                        
                if (*board).has_winner(){
                    we_have_a_winner = true;
                    println!("WINNER!");
                    println!("Number: {} Result: {}",  number_to_mark, (*board).sum_not_marked());
                    break;
                }
            }

            if we_have_a_winner {
                break;
            }
            
        }
    }else{ // Part B
        
        let mut winner_boards = true;
        
        for number in drawn_numbers{
            println!("Number: {}", number);
            let number_to_mark = &number;

            winner_boards = true;
            for board in boards.iter_mut(){
                (*board).mark_number(String::from(number_to_mark));
                                
                let is_a_winner = (*board).has_winner();

                if !is_a_winner{
                    println!("{}",board);                        
                    println!("LOSER!");
                    println!("Number: {} Result: {}",  number_to_mark, (*board).sum_not_marked());
                }

                winner_boards = winner_boards &  is_a_winner
            }

            if winner_boards {
                break;
            }
            
        }

    }
    let result:String = String::from("");
    Ok(result)
}