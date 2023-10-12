mod file_reader;
mod source_lexer;
mod source_parser;

use file_reader::File_Reader;
use source_lexer::Source_Lexer;

struct Source_Reader {
}

impl Source_Reader {
    pub fn read_source(filename: &str) -> CharacterSheet {
        // read contents of file 
        let fr = File_Reader();
        let contents = fr.read_file(filename);
        contents.iter().map(|line| line.as_str()); // convert to Vec<&str>
        // pass contents of file to lexer 
        let lexer = Source_Lexer(contents);
        lexer.lex()
        // pass lexer result to parser 
        // parse tree into character CharacterSheet 
        // return CharacterSheet
    }
}
