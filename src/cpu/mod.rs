use crate::instruction::Instruction;
use crate::program::Program;
use std::mem;

pub struct Cpu {
    pub is_finished: bool,
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
        let is_finished = false;
        Cpu { is_finished, schedule, pipeline, programs, curr_instruction, counter_bubble }
    }

    pub fn move_instruction(&mut self, position: usize) {
        if position < 4 {
            let moved = mem::replace(&mut self.pipeline[position], Box::new(Instruction::Noop));
            self.pipeline[position + 1] = moved;
        } else {
            self.pipeline[position] = Box::new(Instruction::Noop);
        }
    }

    pub fn add_instruction_to_pipeline(&mut self) {
        // mudar quando adicionar jump
        let first = self.programs[self.schedule].instructions.remove(0);
        // implementar futuramente scheduler
        // self.schedule();
        self.pipeline[0] = Box::new(first);
        if self.programs[self.schedule].instructions.is_empty() {
            self.programs.remove(self.schedule);
        }
    }

    pub fn move_pipeline(&mut self) {
        let mut has_moved = false;

        for i in (3..=4).rev() {
            self.move_instruction(i);
        }

        let analyzed_instr = &mut self.pipeline[2];
        match &**analyzed_instr {
            Instruction::Add(_, _, _) => { 
                self.move_instruction(2);
                has_moved = true; 
            }
            _ => (),
        }

        if has_moved {
            for i in (0..=1).rev() {
                self.move_instruction(i);
            }
            if !self.programs.is_empty() {
                self.add_instruction_to_pipeline();
            } else {
                self.is_finished = true;
            }
        }
        self.print_pipeline();
    }

    pub fn print_pipeline(&self) {
        println!("1: {:?};\n 2: {:?};\n 3: {:?};\n 4: {:?};\n 5: {:?}\n\n", self.pipeline[0], self.pipeline[1], self.pipeline[2], self.pipeline[3], self.pipeline[4]);
    }
}
