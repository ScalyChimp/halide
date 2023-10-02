use chumsky::Parser;
use opcodes::instructions::Instruction;
use std::{error::Error, fs, num::ParseIntError, path::PathBuf};

use assembler::assemble;
use clap::Parser as ArgParser;
use vm::VM;
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
        None => repl(args)?,
    }
    Ok(())
}

fn repl(args: Args) -> Result<(), Box<dyn Error>> {
    let mut rl = rustyline::config()?;

    println!("halide-vm repl v0.0.1");
    let mut input = rl.readline(">> ");
    let mut vm = VM::default();
    loop {
        match input {
            Ok(ref line) => {
                if line.is_empty() {
                    input = rl.readline(">> ");
                    continue;
                }

                match line.as_str() {
                    ".step" => vm.step(),
                    ".run" => vm.run(),

                    ".dbg" => {
                        println!("Full VM state:");
                        dbg!(&vm);
                    }
                    ".registers" => {
                        println!("Registers: ");
                        println!("{:#?}", vm.registers)
                    }
                    ".program" => {
                        println!("Program: ");
                        println!("{:#?}", vm.program)
                    }
                    ".quit" => {
                        println!("buh-bye!");
                        std::process::exit(0);
                    }
                    input => match parse_input_to_bytes(input, &args) {
                        Ok(hex) => {
                            print!("Loading hex: ");
                            for byte in hex.iter() {
                                print!("{:#04X} ", byte);
                            }
                            println!();
                            vm.program = hex;
                        }
                        Err(err) => {
                            eprintln!("invalid input: {:?}", err);
                        }
                    },
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

fn parse_input_to_bytes(input: &str, args: &Args) -> Result<Vec<u8>, Box<dyn Error>> {
    if args.raw_hex {
        Ok(parse_hex(input)?)
    } else {
        let instr = assemble().parse(input).unwrap();

        dbg!(&instr);
        let mapped: Vec<_> = instr.into_iter().map(Instruction::to_bytes).collect();
        dbg!(&mapped);
        Ok(mapped.into_iter().flatten().collect())
    }
}

fn parse_hex(input: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut results: Vec<u8> = vec![];
    for hex_string in input.split(' ') {
        match u8::from_str_radix(hex_string, 16) {
            Ok(result) => results.push(result),
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(results)
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
