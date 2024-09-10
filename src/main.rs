use std::{fs::File, io::{self, stdout, BufRead, Write}, path::Path, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use mrt_cpu::{compiler::Compiler, computer::*};


fn main() {
    println!("MRT-CPU CLI Utility
    Enter `help' for a list of commands
    ");

    let mut system = System::new(0);
    
    let interrupt = Arc::new(AtomicBool::new(false));
    
    {
        let interrupt_clone = interrupt.clone();

        ctrlc::set_handler(move || {
            print!("Ctrl-C");
            _ = stdout().flush();
            interrupt_clone.store(true, Ordering::Release)
        }).expect("Error: failed to set interrupt handler");
    }

    loop {
        print!("% ");

        let _ = io::stdout().flush();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut line = String::new();
        handle.read_line(&mut line).expect("Failed to gather input from stdin");

        let len = line.trim_end_matches(&['\r', '\n'][..]).len();
        line.truncate(len);
        let command = line.split(' ').collect::<Vec<&str>>();

        // do something with input
        match command[0] {
            "help" => {
                println!("Help: main, alias [required] <optional> - description
    exit, quit - exit application
    load_rom, lr [rom_file] - load a rom
    ram_size [ram_size] - set ram size
    step, s <step_count> - step N amount of instructions
    continue, c - continue running until Ctrl+C
    compile, com [file] <out> - compile assembly file and output to `out'
    regs - print system registers
                ");
            },
            "exit" | "quit" => {
                return;
            },
            "load_rom" | "lr" => 'load_rom: {
                let path = command.get(1);
                if path.is_none() {
                    println!("Error: No path provided");
                    break 'load_rom;
                }

                let rom = std::fs::read(Path::new(path.unwrap())); 
                if rom.is_err() {
                    println!("Error: Failed to read from file");
                    break 'load_rom;
                }

                let result = system.load_rom(rom.unwrap());
                if result.is_err() {
                    println!("{:?}", result.err().unwrap());
                }
            },
            "ram_size" => 'ram_size: {
                let size = command.get(1);
                if size.is_none() {
                    println!("Error: No size provided");
                    break 'ram_size;
                }

                let size = size.unwrap().parse::<usize>();
                if size.is_err() {
                    println!("Error: Expected an integer for size");
                    break 'ram_size;
                }

                let result = system.load_ram(vec![0; size.unwrap()]);
                if result.is_err() {
                    println!("{:?}", result.err().unwrap());
                }
            },
            "step" | "s" => 'step: {
                let step_count = command.get(1);
                let mut step_count = if step_count.is_some() {
                    let step_count = step_count.unwrap().parse::<usize>();
                    if step_count.is_err() {
                        println!("Error: Expected an integer for step count");
                        break 'step;
                    }

                    step_count.unwrap()
                } else { 1 };
            
                while step_count > 0 {
                    step_count -=1 ;
                    if system.tick() {
                        break;
                    }
                }
            },
            "continue" | "c" => {
                const CHECK_INTERVAL: usize = 1000;
                let mut counter = 0;

                loop {
                    if system.tick() {
                        break;
                    }

                    counter += 1;
                    if counter > CHECK_INTERVAL {
                        counter = 0;
                        if interrupt.load(Ordering::Acquire) {
                            break;
                        }
                    }
                }

                interrupt.store(false, Ordering::Release);
            },
            "compile" | "com" => 'compile: {
                let input_path = command.get(1);
                if input_path.is_none() {
                    println!("Error: No input file provided");
                    break 'compile;
                }

                let output_path = command.get(2);
                let output_path = output_path.unwrap_or(&"out.rom");

                let input_file = File::open(Path::new(input_path.unwrap())); 
                if input_file.is_err() {
                    println!("Error: Failed to read from file");
                    break 'compile;
                }

                let output_file = File::create(Path::new(output_path));
                if output_file.is_err() {
                    println!("Error: Failed to create output file: {}", output_path);
                    break 'compile;
                }

                let mut compiler = Compiler::new(input_file.unwrap(), output_file.unwrap());
                let result = compiler.compile();
                if result.is_ok() {
                    println!("Info: Compilation succesful, written to file: {}", output_path);
                } else {
                    println!("Error: Compilation failed: {:?}", result.err().unwrap());
                }
            },
            "regs" => {
                let regs = system.get_regs();
                for y in 0..4 {
                    for x in 0..4 {
                        let idx = y * 4 + x;

                        if idx < 10 {
                            print!(" ");
                        }

                        print!("r{} = {:#04x} ", idx, regs[idx])
                    }

                    if y == 0 {
                        print!("\tip={:#04x}", system.get_ipc());
                    }

                    println!();
                }
            },
            _ => {
                println!("Unrecognized command");
            }
        }
    }
}
