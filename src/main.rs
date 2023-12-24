mod parser;
mod run;

fn main() {
    let instructions = parser::parse();
    
    let mut code_runner = run::new_code_runnner(instructions);
    code_runner.run();
}
