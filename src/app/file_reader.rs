use std::{fs, io, path::PathBuf};

struct File_Reader {
    filename: &str
}

impl File_Reader {
    pub fn read_file() -> Result<String> {
        let file_path_prefix = "../../source/";
        let full_path = file_path_prefix + filename;
        let input::Result<String> = fs::read_to_string(full_path);
    
        return input; 
    }

    fn get_files_in_dir() -> io::Result<Vec<PathBuf>> {
        let entries = fs::read_dir(filename)?;
        let all: Vec<PathBuf> = entries
            .filter_map(|entry| Some(entry.ok()?.path()))
            .collect();
        Ok(all)
    }
}

