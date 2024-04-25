use crate::types::{TokenLine, TokenProgram};

pub fn tokenize(source: &String) -> TokenProgram {
    let mut program = TokenProgram::new();
    for line in source.lines() {
        let mut token_line = TokenLine::new();
        for raw_token in line.split(" ") {
            if raw_token.trim().len() == 0 {
                continue;
            }

            token_line.add_token(raw_token.trim().to_owned());
        }

        if token_line.tokens.len() > 0 {
            program.add_line(token_line);
        }
    }

    program
}