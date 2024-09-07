use std::{env, io::{self, BufRead, Write}, path::Path};

use mrt_cpu::computer::*;

fn main() {
    println!("MRT-CPU CLI Utility
    Enter `help' for a list of commands
    ");

    let mut system = System::new(0);

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
                println!("Help:
                exit - exit application
                load_rom [rom_file] - load a rom
                ram_size [ram_size] - set ram size
                ");
            },
            "exit" => {
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
            _ => {
                println!("Unrecognized command");
            }
        }
    }
}
