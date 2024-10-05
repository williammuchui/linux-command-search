use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process::exit;

const LINUX_FILE_PATH: &str = "linux";

fn main() {
    let arguments: Vec<String> = args().collect();
    match arguments.as_slice() {
        [_] => handle_list_argument(),
        [_, arg] if arg == "-l" => handle_list_argument(),
        [_, arg, needle] if arg == "-s" => handle_search_argument(needle),
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

fn handle_search_argument(needle: &str) {
    if let Some(haystack) = read_file(LINUX_FILE_PATH) {
        let found: Vec<&str> = haystack
            .lines()
            .filter(|line| line.to_lowercase().contains(&needle.to_lowercase()))
            .collect();

        for line in found {
            print_contents(line);
        }
    }
}

fn print_help() {
    println!("Usage [command] [arguments]");
}

fn exit_with_error(e: &str) {
    eprintln!("ERROR: \n{}", e);
    exit(1);
}

fn print_contents(contents: &str) {
    println!("{}", contents);
}
