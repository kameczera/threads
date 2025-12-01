use crate::instruction::Instruction;
use crate::instruction::exec_instruction;

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub idx_instr: usize,
    pub memory: [i32; 32],
}

impl Program {
    pub fn new(code: &str) -> Program {
        let mut instructions: Vec<Instruction> = Vec::new();
        
        for (idx, line) in code.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
    
            match Instruction::parse_line(line) {
                Ok(instr) => instructions.push(instr),
                Err(e) => println!("Erro na linha {}: {}", idx + 1, e),
            }
        }
        let idx_instr = 0;
        let memory = [0; 32];
        Program { instructions, idx_instr, memory }
    }

    pub fn exec_instruction(&mut self) {
        for mut instr in &self.instructions {
            exec_instruction(&mut instr, &mut self.memory);
        }
        for data in self.memory {
            println!("{} ", data);
        }
    }
}
