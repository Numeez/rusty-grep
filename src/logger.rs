use std::{fs, path::PathBuf};
use dirs_next::home_dir;



pub fn setup_logger() -> Result<PathBuf, Box<dyn  std::error::Error>> {
    let log_file_path =  get_log_file_path()?; 
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info) 
        .chain(fern::log_file(&log_file_path)?)
        .apply()?;
    clean_log_file(&log_file_path)?;
    Ok(log_file_path)
}


fn clean_log_file(path: &PathBuf)->Result<(),Box<dyn std::error::Error>>{
    let metadata = fs::metadata(path)?;
    let file_mb = metadata.len()/(1024u64 * 1024u64);
    if file_mb>=50{
        fs::remove_file(path)?;
    }
    Ok(())
}

fn get_log_file_path()->Result<PathBuf,Box<dyn std::error::Error>>{
    let mut path = home_dir().ok_or("Could not find home directory")?;
    path.push(".rusty_grep");
    std::fs::create_dir_all(&path)?;
    path.push("rusty-grep.log");
    Ok(path)
}