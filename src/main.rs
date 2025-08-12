mod models;
mod print;
mod utils;
mod logger;

use clap::{Parser};
use utils::*;
use print::*;
use logger::setup_logger;

use std::fs;

use crate::models::Config;



fn main()->Result<(),Box<dyn std::error::Error>> {
    setup_logger()?;
    let config = Config::parse();
    if config.show_logs {
         let logs = fs::read_to_string("rusty-grep.log")?;
        println!("Logs:\n{}", logs);
        Ok(())
    }else{
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

}



fn display_result(result:Vec<(String,Vec<(usize,String,usize)>)>,config:&Config){
    for find in result {
        if find.1.len()==0{
                println!("File name: \x1b[1;36m{}\x1b[0m",find.0);
                println!("No match found");
                continue;
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