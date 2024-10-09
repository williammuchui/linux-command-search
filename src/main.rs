use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process::exit;

const LINUX_FILE_PATH: &str = "linux";

fn main() {
    let arguments: Vec<String> = args().collect();
    match arguments.as_slice() {
        [_] => handle_list_argument(),
        [_, arg] if arg == "-l" || arg == "--list" => handle_list_argument(),
        [_, arg, needle] if arg == "-s" || arg == "--search" => handle_search_exact_command(needle),
        [_, arg] if arg == "-h" => print_help(),
        _ => print_help(),
    }
}

fn handle_list_argument() {
    if let Some(contents) = read_file(LINUX_FILE_PATH) {
        print_contents(&contents);
    }
}

fn read_file(filepath: &str) -> Option<String> {
    let mut file = match File::open(filepath) {
        Ok(f) => f,
        Err(e) => {
            exit_with_error(&format!("Error opening file: {}", e));
            return None;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(e) => {
            exit_with_error(&format!("Error Reading file: {}", e));
            None
        }
    }
}

fn handle_search_exact_command(needle: &str) {
    if let Some(haystack) = read_file(LINUX_FILE_PATH) {
        let found: Vec<&str> = haystack
            .lines()
            .filter(|line| {
                let bytes = line.as_bytes();
                let mut first_word = "";
                for (i, &byte) in bytes.iter().enumerate() {
                    if byte == b']' {
                        first_word = &line[1..i];
                    }
                }
                return first_word.contains(&needle.to_lowercase());
            })
            .collect();

        if &found.len() == &0 {
            println!("NO COMMAND FOUND");
            return;
        }

        let total = found.len();
        if total != 1 {
            println!("{total} MATCHING COMMANDS");
        } else {
            println!("1 MATCHING COMMAND");
        }
        for line in &found {
            print_contents(line);
        }
    }
}

fn print_help() {
    println!("Commands: Display and Search through linux commandline commands");
    println!("Usage [-l] [-s <args>] ");
    println!("-l , --list\n\tList all available commands");
    println!("-s , --search <arg>\n\tSearch for argument in commands");
}

fn exit_with_error(e: &str) {
    eprintln!("ERROR: \n{}", e);
    exit(1);
}

fn print_contents(contents: &str) {
    println!("{}", contents);
}
