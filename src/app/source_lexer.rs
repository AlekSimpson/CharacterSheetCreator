mod source_parser;
mod source_reader;

use std::fs;

struct Source_Lexer {}

impl Source_Lexer {
    pub fn new(source: String) -> Self {
        Source_Lexer {}
    }

    pub fn lex_input(input: Vec<&str>) -> Vec<char> {
        let lexed = vec![];
        let char_vec: Vec<char> = vec![];

        for line in input.iter() {
            char_vec: Vec<char> = vec![];
            let string = line.to_string();

            lexed.append(string.chars().collect()); 
        }

        char_vec
    }
}
