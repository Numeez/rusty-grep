


#[derive(Default,Debug)]
pub struct Config {
    pub files:Vec<String>,
    pub pattern :String,
    pub line_number: bool,
    pub ignore_case: bool,
    pub recursive_search:bool,
    pub regex_enable:bool,

}