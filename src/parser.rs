use std::fs::File;
use std::env;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Parser {
    lines:        Vec<Vec<String>>,
    current_line: i32
}

impl Parser {
    fn match_opcode(&self, opcode_str: &String) -> Result<&str, String> {
        match opcode_str.as_str() {
            "ADD" => Ok("1"),
            "SUB" => Ok("2"),
            "STA" => Ok("3"),
            "LDA" => Ok("5"),
            "BRA" => Ok("6"),
            "BRZ" => Ok("7"),
            "BRP" => Ok("8"),
            "INP" => Ok("901"),
            "OUT" => Ok("902"),
            "HLT" | "COB" => Ok("0"),
            "DAT" => Ok(""),
            _ => Err(format!("parser (line {}), invalid opcode", self.current_line))
        }
    }

    fn label_to_mailbox(&self, label: String) -> i32 {
        for (i, line) in self.lines.iter().enumerate() {
            if line[0] == label {
                return i as i32
            }
        }
        panic!("parser (line {}), label does not translate to a valid mailbox", self.current_line);
    }

    fn operation_to_numeric(&mut self, operations: &Vec<String>) -> String {
        self.current_line += 1;

        if self.current_line > 100 { panic!("mailbox limit reached (99), code cannot be longer than 100 lines") }

        let mut numeric = String::from("");
        let mut value = String::from("");
        
        match operations.len() {
            1 => numeric = self.match_opcode(&operations[0]).unwrap().to_string(),
            2 | 3 => {
                match self.match_opcode(&operations[0]) {
                    Ok(n) => {
                        numeric = n.to_string();
                        value = operations[1].to_string()
                    },
                    Err(_) => {
                        numeric = self.match_opcode(&operations[1]).unwrap().to_string();

                        if operations.len() == 3 {
                            value = operations[2].to_string();
                        }
                    }
                }
            }
            _ => panic!("parser (line {}), invalid instruction length", self.current_line)
        }

        if !value.parse::<i32>().is_ok() && !value.is_empty() {
            value = self.label_to_mailbox(value).to_string();
        }

        if value.len() + numeric.len() > 3 {
            panic!("parser (line {}), invalid instruction length", self.current_line)
        }

        for _ in 0..(3 - (value.len() + numeric.len())) {
            numeric.push('0');
        }

        numeric + value.as_str()
    }
}

fn new_parser(lines: Vec<Vec<String>>) -> Parser {
    Parser{lines, current_line: 0}
}

pub fn parse() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("no input file specified") }

    let lines: Vec<Vec<String>> = io::BufReader::new(File::open(&args[1]).unwrap())
        .lines()
        .map(|l| {
            let mut line = l.unwrap().split_whitespace().map(|x| String::from(x)).collect::<Vec<String>>();
            if let Some(index) = line.iter().position(|x| x == "//") {
                line.truncate(index);
            }
            line
        })
        .filter(|x| x.len() != 0)
        .collect();

    let mut parser = new_parser(lines.clone());

    lines.iter().map(|l| parser.operation_to_numeric(l)).collect()
}
