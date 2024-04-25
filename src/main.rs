
mod utils;
mod files;
mod types;
mod tokenizer;
mod lexer;
mod program;

use tokenizer::tokenize;
use utils::get_args;
use files::{read_file, write_file};

use std::process::ExitCode;

use crate::lexer::lexer;

fn main() -> ExitCode {
    let file_paths = get_args();
    if let Err(files_err) = file_paths {
        eprintln!("{}", files_err);
        return ExitCode::FAILURE;
    }

    let (source_path, dest_path) = file_paths.unwrap();
    let source_code = read_file(&source_path);
    if let Err(_) = source_code {
        eprintln!("Failed to read source: {}", source_path);
        return ExitCode::FAILURE;
    }

    let source_code = source_code.unwrap();

    let tokens = tokenize(&source_code);
    //println!("Tokenized Program:\n{}", tokens);

    let symbols = lexer(&tokens);
    //println!("Lexed Program:\n{}", symbols);

    let compile_status = program::execute(&symbols);
    if let Err(e) = compile_status {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }

    // Do stuff

    let result_code = compile_status.unwrap();
    if let Err(e) = write_file(&dest_path, &result_code) {
        eprintln!("Failed to write compiled BFC: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
