use crate::instruction::Instruction;
use crate::program::Program;
use std::mem;
use std::{thread, time};

pub struct Cpu {
    pub is_finished: bool,
    schedule: usize,
    pipeline: [Box<Instruction>; 5],
    pub programs: Vec<Program>,
    curr_instruction: Instruction,
    counter_bubble: u8,
}

impl Cpu {
    pub fn new(programs: Vec<Program>) -> Cpu {
        let curr_instruction = Instruction::Noop;
        let pipeline = std::array::from_fn(|_| Box::new(Instruction::Noop));
        let schedule = 0;
        let counter_bubble = 0;
        let is_finished = programs.is_empty();
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
        let mut curr_program = &mut self.programs[self.schedule];
        let first = curr_program.instructions[curr_program.idx_instr];
        mem::replace(&mut self.pipeline[0], Box::new(first));
        curr_program.idx_instr += 1;
        if curr_program.is_finished() {
            self.programs.remove(self.schedule);
        }
        // implementar futuramente scheduler
        // self.schedule();
    }

    pub fn move_pipeline(&mut self) {  
        // arrastar instruções já executadas 
        for i in (3..=4).rev() {
            self.move_instruction(i);
        }

        let has_moved = match &*self.pipeline[2] {
            Instruction::Add(_, _, _) => { 
                self.move_instruction(2);
                true
            }
            _ => {
                self.move_instruction(2);
                true
            }
        };

        if has_moved {
            for i in (0..=1).rev() {
                self.move_instruction(i);
            }
            if self.programs.is_empty() {
                self.is_finished = true;
            }
        }

        if let Instruction::Noop = *self.pipeline[0] {
            if !self.programs.is_empty() {
                self.add_instruction_to_pipeline();
            } else {
                self.is_finished = true;
                self.print_pipeline();
                return;
            }
        }

        self.print_pipeline();
    }

    pub fn print_pipeline(&self) {
        println!("1: {:?};\n 2: {:?};\n 3: {:?};\n 4: {:?};\n 5: {:?}\n\n", self.pipeline[0], self.pipeline[1], self.pipeline[2], self.pipeline[3], self.pipeline[4]);
    }
}
