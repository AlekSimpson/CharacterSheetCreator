mod source_parser;
mod source_reader;

use std::fs;

struct Source_Lexer {
    input_source: String
}

impl Source_Lexer {
    pub fn new(source: String) -> Self {
        Source_Lexer {
            input_source: source
        }
    }

    pub fn lex_input() -> Vec<Node> {
        
    }


}
