#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add (usize, usize, usize),
    Sub (usize, usize, usize),
    Mul (usize, usize, usize),
    Addi (usize, usize, i32),
    Muli (usize, usize, i32),
    Noop,
}

pub fn exec_instruction(instr: &Instruction, memory: &mut [i32; 32]) {
    match instr {
        Instruction::Add(reg1, reg2, reg3) =>  memory[*reg1] = memory[*reg2] + memory[*reg3],
        Instruction::Sub(reg1, reg2, reg3) =>  memory[*reg1] = memory[*reg2] - memory[*reg3],
        Instruction::Mul(reg1, reg2, reg3) =>  memory[*reg1] = memory[*reg2] * memory[*reg3],
        Instruction::Addi(reg1, reg2, value) => memory[*reg1] = memory[*reg2] + value,
        Instruction::Muli(reg1, reg2, value) => memory[*reg1] = memory[*reg2] * value,
        Instruction::Noop => (),
    }
}

#[derive(Debug)]
pub enum Label {
    Str(String),
    Num(i32),
}

impl Instruction {
    pub fn parse_line(line: &str) -> Result<Instruction, String> {
        let tokens: Vec<&str> = line.split(|c| c == ' ' || c == ',').filter(|s| !s.is_empty()).collect();
        if tokens.is_empty() {
            return Err("Linha vazia".into());
        }

        let name = tokens[0];
        let mut regs = Vec::new();
        for tok in &tokens[1..] {
            match tok.parse::<i32>() {
                Ok(n) => regs.push(n),
                Err(_) => return Err(format!("Argumento inválido: {}", tok)),
            }
        }
        
        if regs.len() != 3 {
            return Err(format!("Instrução {} precisa de 3 argumentos", name))
        }

        let instr = match name {
            "add" => Instruction::Add(regs[0] as usize, regs[1] as usize, regs[2] as usize),
            "addi" => Instruction::Addi(regs[0] as usize, regs[1] as usize, regs[2] as i32),
            "sub" => Instruction::Sub(regs[0] as usize, regs[1] as usize, regs[2] as usize),
            "mul" => Instruction::Mul(regs[0] as usize, regs[1] as usize, regs[2] as usize),
            "muli" => Instruction::Muli(regs[0] as usize, regs[1] as usize, regs[2] as i32),
            "noop" => Instruction::Noop,
            _ => return Err(format!("Instrução {} desconhecida", name)),
        };

        Ok(instr)
    }

    pub fn get_w_register(&self) -> Option<usize> {
        match self {
           Instruction::Noop => None,
           Instruction::Add(r, _, _) 
            | Instruction::Addi(r, _, _)
            | Instruction::Sub(r, _, _)
            | Instruction::Mul(r, _, _)
            | Instruction::Muli(r, _, _) => Some(*r),
        }
    }
}
