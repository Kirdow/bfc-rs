use crate::types::{TokenProgram, Sym, Instruction, Program};

pub fn lexer(tokens: &TokenProgram) -> Program {
    let mut program = Program::new();
    for line in tokens.lines.iter() {
        let first_token = &line.tokens[0];
        let remaining_tokens = &line.tokens[1..];

        let mut inst = Instruction::new();
        inst.add_symbol(Sym::Command(first_token.to_owned()));
        
        for token in remaining_tokens {
            inst.add_symbol(Sym::Word(token.to_owned()));
        }

        inst.add_symbol(Sym::Nop);

        program.add_instruction(inst);
    }

    program
}