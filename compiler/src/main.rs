use chumsky::Parser;
use compiler::{compile_expr, parser};
use std::{error::Error, fs, path::PathBuf};
use vm::opcode::instructions::Instr;
pub use vm::VM;

use clap::Parser as ArgParser;

#[derive(ArgParser)]
struct Args {
    /// File path to bytecode
    /// If ommitted, starts a repl.
    #[arg(short, long, value_name = "BYTECODE")]
    script: Option<PathBuf>,

    #[arg(short, long)]
    raw_hex: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.script {
        Some(script) => run_bytecode(script)?,
        None => repl()?,
    }
    Ok(())
}

fn repl() -> Result<(), Box<dyn Error>> {
    let mut rl = rustyline::config()?;

    println!("halide repl v0.0.1");
    let mut input = rl.readline("- ");
    let mut vm = VM::default();
    loop {
        match input {
            Ok(ref line) => {
                if line.is_empty() {
                    input = rl.readline("- ");
                    continue;
                }

                match line.as_str() {
                    ".step" => vm.step(),
                    ".run" => vm.run(),
                    ".clear" => vm.program = vec![],

                    ".dbg" => {
                        println!("Full VM state:");
                        dbg!(&vm);
                    }
                    ".registers" => {
                        print!("Registers: ");
                        println!("{:?}", vm.registers)
                    }
                    ".program" => {
                        print!("Program: ");
                        println!("{:#?}", vm.program)
                    }
                    ".quit" => {
                        println!("buh-bye!");
                        std::process::exit(0);
                    }
                    input => {
                        let mut hex = parse_input_to_bytes(input);

                        print!("Loading hex: ");
                        for byte in hex.iter() {
                            print!("{:#04X} ", byte);
                            println!();
                        }
                        vm.program.append(&mut hex);
                    }
                }
                rl.add_history_entry(line.as_str())?;
                rl.save_history("halide-vm.history")?;

                input = rl.readline(">> ");
            }
            Err(
                ::rustyline::error::ReadlineError::Eof
                | ::rustyline::error::ReadlineError::Interrupted,
            ) => {
                break Ok(());
            }
            Err(ref err) => print!("Err: {}", err),
        }
    }
}

fn parse_input_to_bytes(input: &str) -> Vec<u8> {
    let input = parser::expr().parse(input).unwrap();

    let bytecode = compile_expr(input, 0);

    bytecode.into_iter().flat_map(Instr::to_bytes).collect()
}

fn run_bytecode(file: PathBuf) -> Result<(), Box<dyn Error>> {
    let str = fs::read_to_string(file)?;
    let hex = str.into_bytes();
    let mut vm = VM::default();
    vm.program = hex;
    vm.run();
    Ok(())
}

mod rustyline {
    use rustyline::{
        error::ReadlineError, highlight::MatchingBracketHighlighter, history::FileHistory,
        validate::MatchingBracketValidator, Editor,
    };
    use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

    #[derive(Completer, Helper, Highlighter, Hinter, Validator)]
    pub struct Config {
        #[rustyline(Validator)]
        brackets: MatchingBracketValidator,
        #[rustyline(Highlighter)]
        highlighting: MatchingBracketHighlighter,
    }

    pub fn config() -> Result<Editor<Config, FileHistory>, ReadlineError> {
        let config = Config {
            brackets: MatchingBracketValidator::new(),
            highlighting: MatchingBracketHighlighter::new(),
        };
        let mut rl = Editor::new()?;
        rl.set_helper(Some(config));
        match rl.load_history("halide-vm.history") {
            Ok(()) => println!("history loaded from halide-vm.history"),
            Err(err) => eprint!("error loading history from halide-vm.history: {}", err),
        }

        Ok(rl)
    }
}
