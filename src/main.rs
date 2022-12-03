use std::{
    env,
    fs::File,
    io::{stdin, Read},
};

const MEMORY_LIMIT: usize = 65535;

fn main() {
    let mut input = String::new();
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: rust-bf <file>");
        return;
    }

    let filename = args.nth(1).unwrap();
    File::open(filename)
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read file");

    let mut memory = [0; MEMORY_LIMIT];
    let mut ptr = 0;

    let chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => memory[ptr] += 1,
            '-' => memory[ptr] -= 1,
            '.' => {
                print!("{}", char::from_u32(memory[ptr]).unwrap())
            }
            ',' => {
                let mut input = String::new();
                stdin()
                    .read_line(&mut input)
                    .expect("Failed to read from stdin");
                memory[ptr] = input.chars().next().unwrap() as u32;
            }
            '[' => {
                if memory[ptr] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        i += 1;
                        match chars[i] {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => (),
                        }
                    }
                }
            }
            ']' => {
                let mut depth = 1;
                while depth > 0 {
                    i -= 1;
                    match chars[i] {
                        '[' => depth -= 1,
                        ']' => depth += 1,
                        _ => (),
                    }
                }
                i -= 1;
            }
            _ => (),
        }
        i += 1;
    }
}
