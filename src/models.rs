use clap::{Parser};


#[derive(Parser,Debug)]
#[command(
    name = "rusty-grep",
    about = "A Rust-powered grep clone",
    author = "M Numeez Baloch",
    version = "0.1.0"

)]
pub struct Config {
    pub pattern :String,
    #[arg(required = true)]
    pub files:Vec<String>,
    #[arg(short = 'n', long)]
    pub line_number: bool,
     #[arg(short = 'i', long)]
    pub ignore_case: bool,
     #[arg(short = 'r', long)]
    pub recursive_search:bool,
      #[arg(short = 'E', long)]
    pub regex_enable:bool,
      #[arg(short = 'H', long)]
    pub attached_header_name:bool,
     #[arg(short = 'c', long)]
    pub show_counts_only:bool,
    #[arg(short = 'l', long)]
    pub show_logs:bool,

}


pub type FileInfo = (usize,String,usize);