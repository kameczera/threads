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

#[derive(Debug)]
enum Instruction {
    Add (i32, i32, i32),
    Sub (i32, i32, i32),
    Mul (i32, i32, i32),
}

#[derive(Debug)]
enum Label {
    Str(String),
    Num(i32),
}

impl Instruction {
    fn parse_line(line: &str) -> Result<Instruction, String> {
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
            "add" => Instruction::Add(regs[0], regs[1], regs[2]),
            "sub" => Instruction::Sub(regs[0], regs[1], regs[2]),
            "mul" => Instruction::Mul(regs[0], regs[1], regs[2]),
            _ => return Err(format!("Instrução {} desconhecida", name)),
        };

        Ok(instr)
    }
}

fn parser(code: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    
    for (idx, line) in code.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        match Instruction::parse_line(line) {
            Ok(instr) => result.push(instr),
            Err(e) => println!("Erro na linha {}: {}", idx + 1, e),
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let contents = fs::read_to_string(config.file_path).expect("Erro lendo o arquivo");
    
    let instructions = parser(&contents);
    println!("Instruções encontradas:");
    for instr in instructions {
        println!("{:?}", instr);
    }
}
