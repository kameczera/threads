use std::env;
use std::fs;

mod cpu;
mod instruction;
mod program;

use instruction::Instruction;
use program::Program;
use crate::cpu::Cpu;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let contents = fs::read_to_string(config.file_path).expect("Erro lendo o arquivo");
    let program = Program::new(&contents);
    let mut programs: Vec<Program> = Vec::new();
    programs.push(program);
    let mut cpu = Cpu::new(programs);
    while cpu.is_finished == false {
        cpu.move_pipeline();
        println!("{:?}", cpu.programs);
    }
}
