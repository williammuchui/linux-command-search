# Linux Command Search

## Description

This project provides a simple command-line tool for searching and displaying Linux commands. It allows users to view a full list of common Linux commands or search for specific commands using keywords.

## Features

- Display a full list of common Linux commands
- Search for commands using case-insensitive keyword matching
- Easy-to-use command-line interface

## Requirements

- C compiler (e.g., gcc)
- Standard C libraries

## Installation

1. Clone the repository or download the source code.
2. Compile the program using a C compiler:
   ```
   gcc -o commands src/main.c
   ```

## Usage

The program can be used in three ways:

1. Display all commands:

   ```
   ./commands
   ```

   or

   ```
   ./commands -l
   ```

2. Search for a specific command:
   ```
   ./commands -s <keyword>
   ```
   Replace `<keyword>` with the term you want to search for.

## File Structure

- `src/main.c`: The main source code file containing the program logic
- `linux`: A text file containing the list of Linux commands and their descriptions

## How It Works

- The program reads the list of commands from `linux`.
- When searching, it uses a case-insensitive search algorithm to match the keyword against command names and descriptions.
- Results are displayed in the terminal.

## Customization

You can easily add or modify commands by editing the `linux` file. Each line should follow this format:

```
[command_name] Command description
```

## Contributing

Contributions to improve the program or expand the command list are welcome. Please submit a pull request or open an issue to discuss proposed changes.

## License

This project is open source and available under the [MIT License](https://opensource.org/licenses/MIT).

## Contact

For any questions or suggestions, please open an issue in the project repository.
