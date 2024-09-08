use std::{io::{self, stdout, BufRead, Write}, path::Path, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use mrt_cpu::computer::*;


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
    load_rom [rom_file] - load a rom
    ram_size [ram_size] - set ram size
    step, s <step_count> - step N amount of instructions
    continue, c - continue running until Ctrl+C
                ");
            },
            "exit" | "quit" => {
                return;
            },
            "load_rom" => 'load_rom: {
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
                    println!("{}", result.err().unwrap());
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
                    println!("{}", result.err().unwrap());
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
                while !interrupt.load(Ordering::Acquire) {
                    if system.tick() {
                        break;
                    }
                }

                interrupt.store(false, Ordering::Release);
            },
            _ => {
                println!("Unrecognized command");
            }
        }
    }
}
