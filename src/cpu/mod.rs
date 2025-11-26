use crate::instruction::Instruction;
use crate::program::Program;
use std::mem;

struct Cpu {
    schedule: usize,
    pipeline: [Box<Instruction>; 5],
    programs: Vec<Program>,
    curr_instruction: Instruction,
    counter_bubble: u8,
}

impl Cpu {
    pub fn new(programs: Vec<Program>) -> Cpu {
        let curr_instruction = Instruction::Noop;
        let pipeline = std::array::from_fn(|_| Box::new(Instruction::Noop));
        let schedule = 0;
        let counter_bubble = 0;
        Cpu { schedule, pipeline, programs, curr_instruction, counter_bubble }
    }

    pub fn move_instruction(&mut self, position: usize) {
        if position < 4 {
            let moved = mem::replace(&mut self.pipeline[position], Box::new(Instruction::Noop));
            self.pipeline[position + 1] = moved;
        } else {
            self.pipeline[position] = Box::new(Instruction::Noop);
        }
    }

    pub fn move_pipeline(&mut self) {
        for i in (3..=4).rev() {
            self.move_instruction(i);
        }

        let analyzed_instr = &mut self.pipeline[2];
        match &**analyzed_instr {
                Instruction::Add(_, _, _) => { 
                    self.move_instruction(2); 
                }
            _ => (),
        }
    }
}
