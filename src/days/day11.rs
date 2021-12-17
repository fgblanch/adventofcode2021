use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn print_octo(octopuses: &[[u16; 10]; 10]){
    for j in 0..10{
        for k in 0..10{ 
            print!("{}\t",octopuses[j][k]);
        }
        print!("\n");
    }
}

fn flash(j:usize,k:usize, octopuses_temp:&mut  [[u16; 10]; 10], flash_counter: &mut u32){
     //flash                    
     *flash_counter+=1;

     if (k>0) & (j>0){
         octopuses_temp[j-1][k-1] += 1;
         if octopuses_temp[j-1][k-1] == 10{
            flash(j-1,k-1, octopuses_temp, flash_counter);
         }
     }
     if j>0{
         octopuses_temp[j-1][k] += 1;
         if octopuses_temp[j-1][k] == 10{
            flash(j-1,k, octopuses_temp, flash_counter);
         }
     }
     if (k<9) & (j>0){
         octopuses_temp[j-1][k+1] += 1;
         if octopuses_temp[j-1][k+1] == 10{
            flash(j-1,k+1, octopuses_temp, flash_counter);
         }
     }

     if k>0{
         octopuses_temp[j][k-1] += 1;
         if octopuses_temp[j][k-1] == 10{
            flash(j,k-1, octopuses_temp, flash_counter);
         }
     }
     if k<9{
         octopuses_temp[j][k+1] += 1;
         if octopuses_temp[j][k+1] == 10{
            flash(j,k+1, octopuses_temp, flash_counter);
         }
     }
     
     if (k>0) & (j<9){
         octopuses_temp[j+1][k-1] += 1;
         if octopuses_temp[j+1][k-1] == 10{
            flash(j+1,k-1, octopuses_temp, flash_counter);
         }
     }
     if j<9{
         octopuses_temp[j+1][k] += 1;
         if octopuses_temp[j+1][k] == 10{
            flash(j+1,k, octopuses_temp, flash_counter);
         }
     }
     if (k<9) & (j<9){
         octopuses_temp[j+1][k+1] += 1;
         if octopuses_temp[j+1][k+1] == 10{
            flash(j+1,k+1, octopuses_temp, flash_counter);
         }
     }            

}


fn day11(input_path:String) -> Result<String, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    

    let mut octopuses: [[u16; 10]; 10] = [[0;10];10];

    let mut line_counter:u16 = 0;
    for line in buffered.lines() {        
        let current:String = line?; 
        for (i,elem) in current.chars().enumerate(){
            octopuses[line_counter as usize][i] = String::from(elem).parse().unwrap();
        }
        line_counter+=1;
    }

    println!("\n Step: {}",0);
    print_octo(&octopuses);

    let mut flash_counter:u32 = 0;
    let mut first_sync:u16 = 0;

    for i in 0..500{ // steps

        let mut octopuses_temp: [[u16; 10]; 10] = [[0;10];10];

        for j in 0..10{
            for k in 0..10{                
                octopuses_temp[j][k] = octopuses_temp[j][k] + octopuses[j][k] +1;
                //println!("j:{} k:{}",j,k);
                if octopuses_temp[j][k] > 9{
                    flash(j,k, &mut octopuses_temp, &mut flash_counter);                           
                }                
                //println!("");
                //print_octo(&octopuses_temp);
            } 
        }
        
        let mut all_in_sync:bool = true;
        
        let mut last_number = octopuses_temp[0][0];

        for j in 0..10{
            for k in 0..10{
                if octopuses_temp[j][k] > 9{
                    octopuses_temp[j][k] = 0                
                }

                all_in_sync = all_in_sync & (last_number == octopuses_temp[j][k]);
                last_number = octopuses_temp[j][k];
            }
        }
        
        println!("\n Step: {}",i+1);
        print_octo(&octopuses_temp);

        octopuses = octopuses_temp;
        
        if all_in_sync & (first_sync==0){
            first_sync = i;
            break
        }

    }
    
   
    let result:String = format!("flash counter:{} first sync: {}",flash_counter, first_sync);

    Ok(result)
}
