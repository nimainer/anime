use std::{path::Path, fs};

pub fn search_file(file_name: &str) -> Option<String> {
    let mut _rs =false;
    let file_path = format!("resources/anime_list/{}.txt", file_name).split_whitespace().collect();
    _rs = Path::new(&file_path).exists();
    
    if _rs == true{
        return Some(format!("Here is the watch order for {}:\n{}", file_name ,file_reader(&file_path)));
    }
    None 
}

fn file_reader(file_path: &String) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents
}