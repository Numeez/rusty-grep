use std::{env, io::Error, fs, vec};

fn main() {
    let arguments:Vec<String> = env::args().collect();
    if arguments.len()<3{
          eprintln!("Usage: {} [FLAGS] <pattern> <file1> [file2 ...]", arguments[0]);
          std::process::exit(1);
    }
    let config = fetch_pattern_and_files(&arguments);
    if let Ok(result) = find_match_in_files(&config.files,&config.pattern,config.ignore_case){
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
}else if let Err(err) = find_match_in_files(&config.files,&config.pattern,config.ignore_case){
    println!("Error occured: {}",err);
    std::process::exit(1)
    
}

}

fn fetch_pattern_and_files(arguments:&Vec<String>)->Config{
    let mut pattern_and_file : Vec<String> = vec![];
    let mut config:Config = Config::default();
    for (i,args )in arguments.iter().enumerate().skip(1){
        if args.starts_with("-"){
            match args.as_str() {
                "-n"=> config.line_number = true,
                "-i"=> config.ignore_case = true,
                _=>{}

            } 
            continue;
        }else{
             pattern_and_file = arguments[i..].to_vec();
             break;
            
        }
    }
    let pattern = &pattern_and_file[0];
    let files:Vec<String> = pattern_and_file[1..].to_vec();
    config.pattern = pattern.to_owned();
    config.files = files;
   return config
}

fn find_match_in_files(file_paths:&Vec<String>,pattern:&String,ignore_case:bool)->Result<Vec<(String,Vec<(usize,String)>)>,Error>{
    let mut result = vec![];
    for file_path in file_paths {
        let matched_lines = find_pattern_in_file(pattern, file_path,ignore_case).unwrap();

        result.push((file_path.to_owned(),matched_lines));
    }
   Ok( result)
}

fn find_pattern_in_file(pattern:&String, file_path:&String,ignore_case:bool)->Result<Vec<(usize,String)>,Error>{
    let file  = fs::read_to_string(file_path).unwrap();
    let mut result   = vec![];
    let lines:Vec<&str> = file.lines().map(|line|line).collect();
      for (num,line)  in lines.iter().enumerate() {
                let highlight_line = highlight_line(pattern, line,ignore_case);
                if highlight_line!=String::new(){
                result.push((num,highlight_line));
                }

        }

 Ok(result)   
}


fn highlight_line(pattern: &String, line: &str,ignore_case:bool) -> String {
    let mut highlighted_line = String::new();
    let mut last_end = 0;
      let (search_line, search_pattern);
    if ignore_case {
        search_line = line.to_lowercase();
        search_pattern = pattern.to_lowercase();
    } else {
        search_line = line.to_string();
        search_pattern = pattern.to_string();
    }
     let match_info: Vec<_> = search_line.match_indices(&search_pattern).collect();
    if match_info.len()!=0{
    for (start, matched) in  match_info{
        highlighted_line.push_str(&line[last_end..start]); 
        highlighted_line.push_str("\x1b[33m");             
        highlighted_line.push_str(&line[start..start+matched.len()]);              
        highlighted_line.push_str("\x1b[0m");              
        last_end = start + matched.len();
    }

    highlighted_line.push_str(&line[last_end..]); 
    highlighted_line
    }else{
        return  String::new();
    }
}

#[derive(Default)]
struct Config {
    line_number: bool,
    ignore_case: bool,
    pattern :String,
    files:Vec<String>

}