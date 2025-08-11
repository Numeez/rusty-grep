mod models;
mod utils;

use std::{env};
use utils::*;



fn main() {
    let arguments:Vec<String> = env::args().collect();
    if arguments.len()<3{
          eprintln!("Usage: {} [FLAGS] <pattern> <file1> [file2 ...]", arguments[0]);
          std::process::exit(1);
    }
    let config = fetch_pattern_and_files(&arguments);

    if let Ok(result) = find_match_in_files(&config.files,&config.pattern,config.ignore_case,config.recursive_search,config.regex_enable){
    for find in result {
        println!();
        println!("File name: \x1b[34m{}\x1b[0m",find.0);
        println!();
        if find.1.len()==0{
                println!("No match found");
            }
         if config.line_number{
        for line in find.1{
            println!("Ln{}: {}",line.0+1,line.1);
            println!();
        }
        println!();
    }else{
        for line in find.1{
            println!("{}",line.1);
            println!();
        }
        println!();

    }
    }
    std::process::exit(0)
}else if let Err(err) = find_match_in_files(&config.files,&config.pattern,config.ignore_case,config.recursive_search,config.regex_enable){
    println!("Error occured: {}",err);
    std::process::exit(1)
    
}

}

