use std::fmt::Display;

pub enum Sym {
    Nop,
    Command(String),
    Word(String),
}

pub struct TokenLine {
    pub tokens: Vec<String>
}

pub struct TokenProgram {
    pub lines: Vec<TokenLine>
}

pub struct Instruction {
    pub symbols: Vec<Sym>
}

pub struct Program {
    pub instructions: Vec<Instruction>
}

impl TokenLine {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new()
        }
    }

    pub fn add_token(&mut self, token: String) {
        self.tokens.push(token);
    }
}

impl TokenProgram {
    pub fn new() -> Self {
        Self {
            lines: Vec::new()
        }
    }

    pub fn add_line(&mut self, line: TokenLine) {
        self.lines.push(line);
    }
}

#[allow(unused)]
impl Instruction {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new()
        }
    }

    pub fn add_symbol(&mut self, sym: Sym) {
        self.symbols.push(sym)
    }

    pub fn get(&self, idx: u64) -> Option<&Sym> {
        self.symbols.get(idx as usize)
    }

    pub fn len(&self) -> u64 {
        self.size() - 1
    }

    pub fn size(&self) -> u64 {
        self.symbols.len() as u64
    }
}

#[allow(unused)]
impl Program {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new()
        }
    }

    pub fn add_instruction(&mut self, inst: Instruction) {
        self.instructions.push(inst)
    }

    pub fn get(&self, line: u64) -> Option<&Instruction> {
        self.instructions.get(line as usize)
    }

    pub fn len(&self) -> u64 {
        self.instructions.len() as u64
    }
}

// Display

impl Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sym::Command(name) => write!(f, "C-{}", name),
            Sym::Word(text) => write!(f, "W-{}", text),
            Sym::Nop => write!(f, "---")
        }
    }
}

impl Display for TokenLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for token in self.tokens.iter() {
            if !first {
                write!(f, " ")?;
            }

            write!(f, "{}", token)?;
            first = false;
        }

        Ok(())
    }
}

impl Display for TokenProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token Lines: {}", self.lines.len())?;

        for line in self.lines.iter() {
            write!(f, "\n  {}", line)?;
        }

        Ok(())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for sym in self.symbols.iter() {
            if !first {
                write!(f, " ")?;
            }

            write!(f, "{}", sym)?;
            first = false;
        }

        Ok(())
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instructions: {}", self.instructions.len())?;

        for inst in self.instructions.iter() {
            write!(f, "\n  {}", inst)?;
        }

        Ok(())
    }
}