
mod lexer;
mod parser;
mod compiler;

use crate::lexer::{lang_lexer, lang_token};
use crate::parser::{lang_parser};
use std::fs;

fn main () {

    /*
        Read code 
    */
    let code = match fs::read_to_string("program.txt") {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read program: {}", e);
            return;
        }
    };

    /*
        Tokenize code
    */
    let lang_lexer::LexerArtifacts {
        tokens,
        errors,
    } = lang_lexer::tokenize(&code);

    if !errors.is_empty() {
        for error in errors {
            println!("{:?}", error);
        }
    }
   
    /*
        Parse tokens
    */
    let lang_parser::ParserArtifacts {
        ast,
        errors,
    } = lang_parser::parse(&tokens);

    println!("{:?}", ast);

    /*
    if !errors.is_empty() {
        for error in errors {
            println!("{:?}", error);
        }
        return;
    }
        */

    /*
        Check ast
         for token in tokens {
        println!("{:?}", token);
    }
    */
}
