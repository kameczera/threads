use std::env;
use std::fs;


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
    Add (i32, i32, i32),
    Sub (i32, i32, i32),
    Mul (i32, i32, i32),
}

enum Label {
    string (String),
    number (i32),
}

impl Instruction {
    fn parse_line(registers_string: &String) -> Result<(&str, Vec<Label>), &str> {
        let mut name: &str;
        let mut init = 0;
        let mut last = 0;
        while last < registers_string.len() { 
            if registers_string.as_bytes()[last] == b' ' {
                name = &registers_string[init..last];
            }
            last += 1;
        }
        let mut regs: Vec<Label> = vec!();
        while last < registers_string.len() {
            if registers_string.as_bytes()[last] == b',' {
                match registers_string[init..last].parse::<i32>() {
                    Ok(num) => regs.push(Label::number(num)),
                    Err(_) => regs.push(Label::string(registers_string[init..last].to_string())),
                }
                init = last + 1;
                last = init;
            }
            last += 1;
        }
        match registers_string[init..last].parse::<i32>() {
            Ok(num) => regs.push(Label::number(num)),
            Err(_) => regs.push(Label::string(registers_string[init..last].to_string())),
        }
        Ok((name, regs))
    }

    pub fn new(s_line: &String, idx_line: i32) -> Result<Instruction, &str> {
        let (name, regs) = Self::parse_line(s_line).unwrap();
        let instruction: Instruction;
        match name {
            "add" => {
                if regs.len() != 3 { return Err("Instrução add na linha {idx_line} errada!") }
                instruction = Instruction::Add(regs[0], regs[1], regs[2]);
                
            },
            "sub" => {
                if regs.len() != 3 { return Err("Instrução sub na linha {idx_line} errada!") }
                instruction = Instruction::Sub(regs[0], regs[1], regs[2]);
            },
            "mul" => {
                if regs.len() != 3 { return Err("Instrução mul na linha {idx_line} errada!") }
                instruction = Instruction::Mul(regs[0], regs[1], regs[2]);
            },
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

    parser(&contents);
    println!("With text:\n{contents}");
}
