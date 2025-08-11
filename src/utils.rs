use crate::models::Config;
use regex::Regex;
use walkdir::WalkDir;
use std::{fs, vec};

pub fn fetch_pattern_and_files(arguments:&Vec<String>)->Config{
    let mut pattern_and_file : Vec<String> = vec![];
    let mut config:Config = Config::default();
    for (i,args )in arguments.iter().enumerate().skip(1){
        if args.starts_with("-"){
            match args.as_str() {
                "-n"=> config.line_number = true,
                "-i"=> config.ignore_case = true,
                "-r"=>config.recursive_search = true,
                "-E"=> config.regex_enable = true,
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

pub fn find_match_in_files(file_paths:&Vec<String>,pattern:&String,ignore_case:bool,recursive_search:bool,regex_enable:bool)->Result<Vec<(String,Vec<(usize,String)>)>,Box<dyn std::error::Error>>{
    let mut result = vec![];
    if recursive_search {
        for file_path in file_paths {
        for entry in WalkDir::new(file_path){
            let entry = entry?;
                if entry.file_type().is_file() {
                    let file_path_str = entry.path().to_string_lossy().to_string();
                    let matched_lines = find_pattern_in_file(pattern, &file_path_str,ignore_case,regex_enable)?;
                    result.push((file_path_str.to_owned(),matched_lines));
                }
        }
         }
    }else{
    for file_path in file_paths {
        let matched_lines = find_pattern_in_file(pattern, file_path,ignore_case,regex_enable)?;

        result.push((file_path.to_owned(),matched_lines));
    }
}
   Ok( result)
}

pub fn find_pattern_in_file(pattern:&String, file_path:&String,ignore_case:bool,regex_enable:bool)->Result<Vec<(usize,String)>,Box<dyn std::error::Error>>{
    let file  = fs::read_to_string(file_path)?;
    let mut result   = vec![];
    let lines:Vec<&str> = file.lines().map(|line|line).collect();
      for (num,line)  in lines.iter().enumerate() {
                    if regex_enable {
                        let highlight_line = highlight_line_regex(pattern, line,ignore_case)?;
                if highlight_line!=String::new(){
                result.push((num,highlight_line));
                }
                    }else{
                let highlight_line = highlight_line(pattern, line,ignore_case);
                if highlight_line!=String::new(){
                result.push((num,highlight_line));
                }
            }

        }

 Ok(result)   
}


pub fn highlight_line(pattern: &String, line: &str,ignore_case:bool) -> String {
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


pub fn highlight_line_regex(pattern: &String,line:&str,ignore_case:bool)->Result<String,Box<dyn std::error::Error>>{
     let mut highlighted_line = String::new();
     let mut last_end = 0;
     let mut found_match= false;

    let regex_pattern = if ignore_case {
         format!("(?i){}", pattern)
    }else{
        pattern.to_string()
    };
    let re = Regex::new(&regex_pattern)?;
    for mat in  re.find_iter(line){
        let start = mat.start();
        let end = mat.end();
        found_match = true;
        highlighted_line.push_str(&line[last_end..start]); 
        highlighted_line.push_str("\x1b[33m");             
        highlighted_line.push_str(&line[start..end]);              
        highlighted_line.push_str("\x1b[0m");              
        last_end = end;

    }
    if found_match {
        highlighted_line.push_str(&line[last_end..]);
        Ok(highlighted_line)
    } else {
        Ok(String::new())
    }
}