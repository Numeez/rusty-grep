use crate::models::{Config,FileInfo};



pub fn print_header_result(data:(String,Vec<FileInfo>),config:&Config){
    for line in data.1{
                if config.line_number {
            println!("\x1b[1;36m{}\x1b[0m Ln{}: {}",data.0,line.0+1,line.1);
            println!();
                }else{
                    println!("\x1b[1;36m{}\x1b[0m: {}",data.0,line.1);
            println!();
                }
}
}

pub fn print_count_only_result(data:(String,Vec<FileInfo>)){
    let total_match:usize = data.1.iter().map(|v|v.2).sum();
            println!("\x1b[1;36m{}\x1b[0m: {}",data.0,total_match);
            println!();
}

pub fn print_result(data:(String,Vec<FileInfo>),config:&Config){
    println!();
        println!("File name: \x1b[1;36m{}\x1b[0m",data.0);
        println!();
        if config.line_number{
            print_line_number_result(data)
    }else{
        print_normal_result(data);
    }
}


fn print_line_number_result(data:(String,Vec<FileInfo>)){
for line in data.1{
            println!("\x1b[32mln {}\x1b[0m: {}",line.0+1,line.1);
        }
}

 fn print_normal_result(data:(String,Vec<FileInfo>)){
        for line in data.1{
            println!("{}",line.1);
        }
        println!();
}