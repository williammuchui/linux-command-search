use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process::exit;
const LINUX_FILE_PATH: &str = "linux";

fn main() {
    let arguments = args().collect::<Vec<String>>();
    let args_len = arguments.len();

    if args_len == 1 {
        handle_list_argument();
        return;
    }

    if args_len == 2 && arguments[1] == "-l" {
        handle_list_argument();
        return;
    }

    if args_len == 3 && arguments[1] == "-s" {
        handle_search_argument(&arguments[2]);
        return;
    }

    print_help();
}

fn handle_list_argument() {
    //read file and display contents
    if let Some(contents) = read_file(&LINUX_FILE_PATH) {
        print_contents(&contents);
        return;
    }
    return;
}

fn read_file(filepath: &str) -> Option<String> {
    let mut file;
    match File::open(filepath) {
        Ok(f) => {
            file = f;
        }
        Err(e) => {
            exit_with_error(&format!("Error opening file: {}", e));
            return None;
        }
    }

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(e) => {
            exit_with_error(&format!("Error Reading file: {}", e));
            return None;
        }
    }
}

fn handle_search_argument(needle: &str) {
    let haystack;
    match read_file(&LINUX_FILE_PATH) {
        Some(c) => {
            haystack = c;
        }
        None => {
            return;
        }
    }

    let found = haystack
        .lines()
        .filter_map(|line| {
            if line.contains(needle) {
                Some(line)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();
    for line in found {
        print_contents(&line);
    }
    return;
}

fn print_help() {
    println!("Usage [command] [arguments]");
    return;
}

fn exit_with_error(e: &str) {
    eprintln!("ERROR: \n{}", e);
    exit(1);
}

fn print_contents(contents: &str) {
    println!("{contents}");
    return;
}
