use std::{env::args, fs::File, io::{Error, Read, Write}, process::ExitCode};

fn read_source(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open BFC file!");
    let mut result: String = String::new();
    file.read_to_string(&mut result).expect("Failed to read BFC file!");
    result
}

fn write_compiled(file_path: &str, content: &String) -> Result<(), Error> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <source> <dest>", &args[0]);
        return ExitCode::FAILURE;
    }

    let source_path = &args[1];
    let dest_path = &args[2];

    let source_code = read_source(&source_path);
    if let Err(e) = write_compiled(dest_path, &source_code) {
        eprintln!("Failed to compile BFC: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
