use std::env;
use std::fs;


// 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
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

struct Instruction {
    name: String,
    registers: (i32, i32, i32),
}

impl Instruction {
    pub fn new(name: &str, registers: &str) {
        Instruction { name.to_string(), }
    }

    fn parse_reg(registers: &str) {
        for i in 0..registers.len() {
            if registers[i] == ',' {

            }
        }
    }
}

fn create_instruction(name: &str, registers: &str) -> Instruction {

}

fn parser(&String code) {
    let init = 0;
    let last = 0;
    let name: ;
    let registers;
    let tokens: Vec<Instruction>;
    for i in 0..code.len() {
        if code[i] == ' ' {
            last = i;
            name = &code[init..last];
            init = i + 1;
        } else if code[i] == '\n' {
            registers = &code[init..last];
            last = i;
            tokens.push(create_instruction(name, registers));
            init = i + 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read this file");

    parser(contents);
    println!("With text:\n{contents}");
}
