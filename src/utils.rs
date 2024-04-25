use std::env::args;

pub fn get_args() -> Result<(String, String), String> {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        Err(format!("Usage: {} <source> <dest>", &args[0]))
    } else {
        Ok((args[1].to_owned(), args[2].to_owned()))
    }
}