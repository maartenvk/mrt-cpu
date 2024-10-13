use mrt_cpu::cli::Cli;

use std::{
    io::{self, stdout, BufRead, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

fn main() {
    println!(
        "MRT-CPU CLI Utility
    Enter `help' for a list of commands
    "
    );

    let interrupt = Arc::new(AtomicBool::new(false));

    {
        let interrupt_clone = interrupt.clone();

        ctrlc::set_handler(move || {
            print!("Ctrl-C");
            _ = stdout().flush();
            interrupt_clone.store(true, Ordering::Release)
        })
        .expect("Error: failed to set interrupt handler");
    }

    let mut cli = Cli::new(interrupt.clone());

    loop {
        print!("% ");

        let _ = io::stdout().flush();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut line = String::new();
        handle
            .read_line(&mut line)
            .expect("Failed to gather input from stdin");

        let len = line.trim_end_matches(&['\r', '\n'][..]).len();
        line.truncate(len);
        let command = line.split(' ').collect::<Vec<&str>>();

        // do something with input
        let result = match command[0] {
            "help" => {
                println!(
                    "Help: main, alias [required] <optional> - description
    exit, quit - exit application
    load_rom, lr [rom_file] - load a rom
    ram_size [ram_size] - set ram size
    step, s <step_count> - step N amount of instructions
    continue, c - continue running until Ctrl+C
    compile, com [file] <out> - compile assembly file and output to `out'
    regs - print system registers
    goto [address] - set ip to address
    disassemble, dis <count|from> <to> - disassemble N instruction at ip or from range
    write, w [address] [byte] <count> - write byte N times at address in memory
    read, r [address] <count> - read N bytes from address in memory
                "
                );
                Ok(())
            }
            "exit" | "quit" => return,

            "load_rom" | "lr" => cli.load_rom(command),

            "ram_size" => cli.ram_size(command),

            "step" | "s" => cli.step(command),

            "continue" | "c" => cli.continue_exec(),

            "compile" | "com" => cli.compile(command),

            "regs" => cli.print_regs(),

            "goto" => cli.goto(command),

            "disassemble" | "dis" => cli.disassemble(command),

            "read" | "r" => cli.read_memory(command),

            "write" | "w" => cli.write_memory(command),

            _ => {
                println!("Unrecognized command");
                Ok(())
            }
        };

        if let Err(error) = result {
            println!("{:?}", error);
        }
    }
}
