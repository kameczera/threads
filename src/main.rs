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

enum Instruction {
    add (i32, i32, i32),
    sub (i32, i32, i32),
    mul (i32, i32, i32),
}

enum Label {
    string (String),
    number (i32),
}

impl Instruction {
    fn parse_reg(registers_string: &str) -> Result<(&str, Vec<Label>), &str> {
        let regs: Vec<Label> = vec!();
        let mut init = 0;
        let mut last = 0;
        while i < registers_string.len() {
            if registers_string[i] == ',' {
                match registers_string[init..last].parse::<i32>() {
                    Ok(num) => regs.push(Label::number(num)),
                    Err(_) => regs.push(Label::string(registers_string[init..last].clone())),
                }
            }
            i += 1;
        }
        Ok(regs)
    }

    pub fn new(s_line: &str, idx_line: i32) -> Result<Instruction, &str> {
        let regs = parse_line(s_line).unwrap();
        let instruction: Instruction;
        match name {
            "add" => {
                if regs.len() != 3 { return Err("Instrução linha {idx_line} errado!") }
                instruction = Instruction::add(regs[0], regs[1], regs[2])
                
            },
            "sub" => instruction = Instruction::sub(regs[0], regs[1], regs[2]),
            "mul" => instruction = Instruction::mul(regs[0], regs[1], regs[2]),
        }
        Ok(instruction)
    }
}

fn parser(code: &String) {
    let mut init = 0;
    let mut last = 0;
    let mut curr_line: &str;
    let registers_string;
    let tokens: Vec<Instruction>;
    let mut idx_line = 0;
    for i in 0..code.len() {
        if code[i] == '\n' {
            last = i - 1;
            curr_line = &code[init..last];
            tokens.push(Instruction::new(curr_line, idx_line));
            idx_line += 1;
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
