use std::env::args;

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
    return;
}

fn handle_search_argument(needle: &str) {
    println!("{needle}");
    return;
}

fn print_help() {
    return;
}
