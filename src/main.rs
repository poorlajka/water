
mod lexer;
mod parser;
mod compiler;
mod vm;

use crate::lexer::{lang_lexer, lang_token};
use crate::parser::{lang_parser};
use crate::compiler::{lang_compiler};
use crate::vm::{lang_vm};

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

    /*
    if !errors.is_empty() {
        for error in errors {
            println!("{:?}", error);
        }
    }
        */

    //println!("{:?}", ast);

    /*  
        Compile AST to bytecode
    */
    let lang_compiler::CompilerArtifacts {
        bytecode,
        errors,
    } = lang_compiler::compile(&ast);

    /*  
        Execute bytecode
    */
    let exit_status = lang_vm::exec(&bytecode);
}
