use std::io::{self, Write};
use itertools::{Itertools, EitherOrBoth::*};
use std::process::Command;

#[derive(Debug)]
pub struct Runner {
    pc:           i32,
    acc:          i32,
    instructions: Vec<String>,
    exit_flag:    bool,
    input:        Vec<String>,
    output:       Vec<String>
}

impl Runner {
    fn fetch_mailbox_value(&self, instruction: String) -> i32 {
        self.instructions[instruction[1..].parse::<i32>().unwrap() as usize].parse::<i32>().unwrap()
    }

    fn clear_screen(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/c", "cls"])
                .spawn()
                .expect("cls command failed to start")
                .wait()
                .expect("failed to wait");
        } else {
            Command::new("clear")
                .spawn()
                .expect("clear command failed to start")
                .wait()
                .expect("failed to wait");
        };
    }

    fn render_io_boxes(&self) {
        self.clear_screen();

        println!("INPUT          OUTPUT");
        println!("----------------------");

        for pair in self.input.iter().zip_longest(self.output.iter()) {
            let mut inp = "";
            let mut out = "";

            match pair {
                Both(l, r) => {
                    inp = l;
                    out = r;
                },
                Left(l) => inp = l,
                Right(r) => out = r,
            }

            let mut space = String::from("");

            for _ in 0..(20 - (inp.len() + out.len())) {
                space.push(' ');
            }

            println!(" {}{}{} ", inp, space, out);
        }
    }

    fn take_user_input(&mut self) {
        print!("INP: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if !input.trim().parse::<i32>().is_ok() {
            panic!("non-integer supplied, ({})", input.trim());
        }

        self.acc = input.trim().parse::<i32>().unwrap();
    }

    fn perform_instruction(&mut self, instruction: String) {
        if instruction.len() != 0 {
            match instruction.chars().collect::<Vec<char>>()[0] {
                '1' => self.acc += self.fetch_mailbox_value(instruction),
                '2' => self.acc -= self.fetch_mailbox_value(instruction),
                '3' => {
                    let index = instruction[1..].parse::<i32>().unwrap() as usize;
                    self.instructions[index] = self.acc.to_string();
                },
                '5' => self.acc = self.fetch_mailbox_value(instruction),
                '6' => self.pc = instruction[1..].parse::<i32>().unwrap(),
                '7' => if self.acc == 0 { self.pc = instruction[1..].parse::<i32>().unwrap() },
                '8' => if self.acc >= 0 { self.pc = instruction[1..].parse::<i32>().unwrap() },
                '0' => self.exit_flag = true,
                _ => {
                    if instruction == "901" {
                        self.take_user_input();
                        self.input.push(self.acc.to_string());
                    } else if instruction == "902" {
                        self.output.push(self.acc.to_string());
                    }

                    self.render_io_boxes();
                }
            }
        }
    }

    pub fn run(&mut self) {
        while self.pc != self.instructions.len() as i32 {
            if self.exit_flag { return }

            let start_location = self.pc;

            self.perform_instruction(self.instructions[self.pc as usize].clone());

            if self.acc > 999 {
                panic!("value of the accumulator is greater than 999 (max)");
            } else if self.pc == start_location { self.pc += 1; }
        }
    }
}

pub fn new_code_runnner(instructions: Vec<String>) -> Runner {
    Runner{pc: 0, acc: 0, instructions, exit_flag: false, input: Vec::new(), output: Vec::new()}
}
