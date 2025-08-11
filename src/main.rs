mod models;
mod print;
mod utils;

use std::{env};
use utils::*;
use print::*;

use crate::models::Config;



fn main() {
    let arguments:Vec<String> = env::args().collect();
    if arguments.len()<3{
          eprintln!("Usage: {} [FLAGS] <pattern> <file1> [file2 ...]", arguments[0]);
          std::process::exit(1);
    }
    let config = fetch_pattern_and_files(&arguments);
    
    let result = find_match_in_files(&config);
    match  result {
        Ok(result)=>{
            display_result(result, &config);
            std::process::exit(0)
        },
        Err(err)=>{
            println!("Error occured: {}",err);
            std::process::exit(1)
        }
    }

}



fn display_result(result:Vec<(String,Vec<(usize,String,usize)>)>,config:&Config){
    for find in result {
        if find.1.len()==0{
                println!("No match found");
        }
        if config.attached_header_name{
            print_header_result(find,&config)
        }
        else if config.show_counts_only{
            print_count_only_result(find)
       
        }else{
            print_result(find,&config);
        }
    }
}