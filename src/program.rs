use std::fmt::Write;

use crate::types::{Program, Sym};

#[allow(unused)]
mod program_error {
    pub fn on_line_raw<S>(line: u64, err: S) -> String where S: ToString {
        format!("Error on line {}: {}", line, err.to_string())
    }

    pub fn on_line<T, S>(line: u64, err: S) -> Result<T, String> where S: ToString {
        Err(on_line_raw(line, err))
    }

    pub fn generic_raw<S>(err: S) -> String where S: ToString {
        err.to_string()
    }

    pub fn generic<T, S>(err: S) -> Result<T, String> where S: ToString {
        Err(generic_raw(err))
    }

    pub fn writec_raw() -> String {
        String::from("Failed to write compiled code")
    }

    pub fn writec<T>() -> Result<T, String> {
        Err(writec_raw())
    }
}

pub fn get_command(sym: Option<&Sym>, line: u64) -> Result<&String, String> {
    match sym {
        Some(Sym::Command(name)) => Ok(name),
        _ => program_error::on_line(line, "Failed to fetch command")
    }
}

pub fn get_word(sym: Option<&Sym>, line: u64) -> Result<&String, String> {
    match sym {
        Some(Sym::Word(text)) => Ok(text),
        _ => program_error::on_line(line, "Failed to fetch word")
    }
}

pub fn execute(program: &Program) -> Result<String, String> {
    let mut line: u64 = 0;

    let mut result = String::new();

    loop {
        let inst = program.get(line);
        if let None = inst {
            break;
        }
        let inst = inst.unwrap();

        let name = get_command(inst.get(0), line)?;
        if name == "put" {
            let mode = get_word(inst.get(1), line)?;
            if mode != "string" {
                return program_error::on_line(line, format!("Unexpected {}, expected string!", mode));
            }

            let str_to_put = get_word(inst.get(2), line)?;

            execute_put_string(&mut result, str_to_put)?;
        }

        line += 1;
    }
    
    Ok(result)
}

fn execute_put_string(dataresult: &mut String, str_to_put: &String) -> Result<(), String> {
    let mut prev = 0;
    for c in str_to_put.chars() {
        let current_value = (c as u8) as i32;
        let diff = current_value - prev;

        let target = if diff < 0 {
            '-'
        } else {
            '+'
        };

        for _ in 0..diff.abs() {
            dataresult.write_char(target).map_err(|_| program_error::writec_raw())?;
        }

        dataresult.write_char('.').map_err(|_| program_error::writec_raw())?;
        prev = current_value;
    }

    Ok(())
}