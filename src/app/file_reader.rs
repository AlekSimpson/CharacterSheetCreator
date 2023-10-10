struct File_Reader {
    filename: String
}

impl File_Reader {
    pub fn read_file() -> Result<String> {
        let file_path_prefix = "../../source/";
        let full_path = file_path_prefix + filename;
        let input::Result<String> = fs::read_to_string(full_path);
    
        return input; 
    }

    pub fn get_sheets() -> Vec<String> {

    }
}

