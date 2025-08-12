use crate::models::{Config,FileInfo};
use regex::Regex;
use walkdir::{WalkDir,DirEntry};
use std::{fs::{self, File}, io::{BufRead, BufReader}, vec};


pub fn find_match_in_files(config:&Config)->Result<Vec<(String,Vec<FileInfo>)>,Box<dyn std::error::Error>>{
    let mut result = vec![];
    let file_paths = &config.files;
    if config.recursive_search {
        for file_path in file_paths {
        for entry in WalkDir::new(file_path){
            let entry = entry?;
                if is_skippable(&entry){
                    continue;
                }
                if entry.file_type().is_file() {
                    let file_path_str = entry.path().to_string_lossy().to_string();
                    let matched_lines = find_pattern_in_file(&config.pattern, &file_path_str,config.ignore_case,config.regex_enable)?;
                    result.push((file_path_str.to_owned(),matched_lines));
                }
        }
         }
    }else{
    for file_path in file_paths {
        let matched_lines = find_pattern_in_file(&config.pattern, &file_path,config.ignore_case,config.regex_enable)?;
        if !matched_lines.is_empty(){
        result.push((file_path.to_owned(),matched_lines));
        }
    }
}
   Ok( result)
}

pub fn find_pattern_in_file(pattern:&String, file_path:&String,ignore_case:bool,regex_enable:bool)->Result<Vec<FileInfo>,Box<dyn std::error::Error>>{
    let file  = fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let  result:Vec<_>;
    if regex_enable {
    result = fetch_regex_result(pattern, &mut reader, ignore_case)?;
    }else{
       result =  fetch_result(pattern, &mut reader, ignore_case)?;
    }
  Ok(result)
}


pub fn highlight_line(pattern: &str, line: &str,ignore_case:bool) -> (String,usize) {
    let mut highlighted_line = String::new();
    let mut last_end = 0;
    let mut word_count =0;
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
        word_count+=1;
        highlighted_line.push_str(&line[last_end..start]); 
        highlighted_line.push_str("\x1b[33m");             
        highlighted_line.push_str(&line[start..start+matched.len()]);              
        highlighted_line.push_str("\x1b[0m");              
        last_end = start + matched.len();
    }

    highlighted_line.push_str(&line[last_end..]); 
    (highlighted_line,word_count)
    }else{
        return  (String::new(),0);
    }
}


pub fn highlight_line_regex(regex: &Regex,line:&str)->Result<(String,usize),Box<dyn std::error::Error>>{
     let mut highlighted_line = String::new();
     let mut last_end = 0;
     let mut found_match= false;
     let mut word_count  = 0;   

    for mat in  regex.find_iter(line){
        word_count+=1;
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
        Ok((highlighted_line,word_count))
    } else {
        Ok((String::new(),0))
    }
}

fn is_skippable(dir:&DirEntry)->bool{
    let path = dir.path();
    let file_type = dir.file_type();
    if file_type.is_dir(){
       if let Some(name) =  path.file_name().and_then(|n| n.to_str()){
        return name==".git" || name==".node_modules" || name=="target";
       }
    }else if file_type.is_file(){
        if let Some(extension) = path.extension().and_then(|e|e.to_str()){
        let skip_exts = ["pdf", "exe", "dll", "bin", "jpg", "png", "gif"];
       return  skip_exts.contains(&extension.to_lowercase().as_str());
        }
    }
    false
    }


fn fetch_regex_result(pattern:&str,reader:&mut BufReader<File>,ignore_case:bool)->Result<Vec<FileInfo>,Box<dyn std::error::Error>>{
    let mut result = vec![];
    let regex_pattern = if ignore_case {
                    format!("(?i){}", pattern)
                    }else{
                        pattern.to_string()
                 };
    let re = Regex::new(&regex_pattern)?;
        for (num,line)  in reader.lines().enumerate(){
        let line = match line {
            Ok(l)=>l,
            Err(err)=>{
                 eprintln!("error encountered:{}",err);
                 continue;
            }
        };
        let highlight_line = highlight_line_regex(&re, &line)?;
                if highlight_line.0!=String::new(){
                result.push((num,highlight_line.0,highlight_line.1));
                }
            }
            Ok(result)
}
fn fetch_result(pattern:&str,reader:&mut BufReader<File>,ignore_case:bool)->Result<Vec<FileInfo>,Box<dyn std::error::Error>>{
    let mut result = vec![];
        for (num,line)  in reader.lines().enumerate(){
        let line = line?;
        
        {
                let highlight_line = highlight_line(pattern, &line,ignore_case);
                if highlight_line.0!=String::new(){
                result.push((num,highlight_line.0,highlight_line.1));
                }
            }

        }
            Ok(result)
}