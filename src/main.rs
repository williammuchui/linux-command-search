use std::env::{args, var};
use std::fs::File;
use std::io::{self as std_io, Read};
use std::path::PathBuf;
use std::process::{exit, Command};
use term_size::dimensions;

fn main() {
    draw_commands_ascii();
    let arguments: Vec<String> = args().collect();
    match arguments.as_slice() {
        [_] => handle_list_argument(),
        [_, arg] if arg == "-l" || arg == "--list" => handle_list_argument(),
        [_, arg, needle] if arg == "-s" || arg == "--search" => handle_search_exact_command(needle),
        [_, arg] if arg == "-h" || arg == "--help" => print_help(),
        _ => print_help(),
    }
}

fn handle_list_argument() {
    let filepath = get_file_path();
    if let Some(contents) = read_file(&filepath) {
        let lines: Vec<&str> = contents.lines().collect();
        gradual_print(&lines);
    } else {
        exit_with_error("Failed to read commands from file.");
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

fn gradual_print(lines: &[&str]) {
    let (_width, height) = dimensions().unwrap_or((80, 20)); // Fallback to 80x20
    let max_lines = height - 1; // Reserve space for user input
    let mut index = 0;
    while index < lines.len() {
        for i in index..(index + max_lines).min(lines.len()) {
            println!("{}", lines[i]);
        }
        if index + max_lines >= lines.len() {
            break; // No more lines to print
        }
        let mut buffer = [0; 1];
        println!("Press 'q' to quit or any other key to continue...");
        std_io::stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] == b'q' {
            break;
        }
        index += max_lines;
    }
}

fn handle_search_exact_command(needle: &str) {
    let filepath = get_file_path();
    if let Some(haystack) = read_file(&filepath) {
        let found: Vec<&str> = haystack
            .lines()
            .filter(|line| line.to_lowercase().contains(&needle.to_lowercase()))
            .collect();
        if found.is_empty() {
            scrap_man(needle);
            return;
        }
        let total = found.len();
        println!(
            "{} MATCHING COMMAND{}",
            total,
            if total == 1 { "" } else { "S" }
        );
        println!("(*) MEANS THE COMMAND RUN BY ROOT USER");
        for line in found {
            print_contents(line);
        }
    }
}

fn get_file_path() -> String {
    let home_dir = var("HOME").unwrap_or_else(|_| String::from("."));
    let mut path = PathBuf::from(home_dir);
    path.push("commands/linux");
    path.to_str().unwrap().to_string()
}

fn print_help() {
    println!("Commands: Display and Search through linux commandline commands");
    println!("Usage: [-l] [-s <args>] [-h]");
    println!("-l , --list\n\tList all available commands");
    println!("-s , --search <arg>\n\tSearch for argument in commands");
    println!("-h , --help\n\tDisplay this help message");
}

fn exit_with_error(e: &str) {
    eprintln!("ERROR: \n{}", e);
    exit(1);
}

fn print_contents(contents: &str) {
    println!("{}", contents);
}

fn draw_commands_ascii() {
    let text = "
 ██████╗ ██████╗ ███╗   ███╗███╗   ███╗ █████╗ ███╗   ██╗██████╗ ███████╗
██╔════╝██╔═══██╗████╗ ████║████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝
██║     ██║   ██║██╔████╔██║██╔████╔██║███████║██╔██╗ ██║██║  ██║███████╗
██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██╔══██║██║╚██╗██║██║  ██║╚════██║
╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║██████╔╝███████║
 ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝
                                                                         
";
    println!("{}", text);
}

fn scrap_man(unknown_command: &str) {
    let man_output = Command::new("man")
        .arg(unknown_command)
        .output()
        .expect("man command failed to start. Is it installed?");
    let man_result =
        String::from_utf8(man_output.stdout).unwrap_or(String::from("Failed to decode"));
    let mut a = [0, 0];
    a[0] = man_result
        .split('\n')
        .into_iter()
        .position(|el| (el == "\u{1b}[1mNAME\u{1b}[0m") || (el == "\u{1b}[1mName\u{1b}[0m"))
        .unwrap();
    a[1] = man_result
        .split('\n')
        .into_iter()
        .position(|el| (el == "\u{1b}[1mSYNOPSIS\u{1b}[0m") || (el == "\u{1b}[1mSynopsis\u{1b}[0m"))
        .unwrap();

    let command_description: String = man_result.split('\n').collect::<Vec<&str>>()[a[0] + 1..a[1]]
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("{}", command_description.trim());
}
