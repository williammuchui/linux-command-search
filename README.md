# Linux Command Search

## Description

This project provides a simple command-line tool for searching and displaying Linux commands. It allows users to view a full list of common Linux commands or search for specific commands using keywords.

## Features

- Display a full list of common Linux commands
- Search for commands using case-insensitive keyword matching
- Easy-to-use command-line interface

## Requirements

- GCC (GNU Compiler Collection)
- Make

## Installation

### Downloading the Source Code

1. Clone the repository:

   ```
   git clone https://github.com/williammuchui/linux-command-search.git
   ```

2. Navigate to the project directory:
   ```
   cd linux-command-search
   ```

### Building the Project

The project uses a Makefile to simplify the build process. Follow these steps to build the project:

1. Compile the program:

   ```
   make
   ```

   This will create the `commands` executable in the project root directory.

2. (Optional) Install the program system-wide:
   ```
   sudo make install
   ```
   This will copy the `commands` executable to `/usr/local/bin`, making it accessible from anywhere in the system.

### Cleaning Up

If you need to clean the build files:

```
make clean
```

### Uninstalling

If you've installed the program system-wide and want to remove it:

```
sudo make uninstall
```

## Usage

After building (and optionally installing) the program, you can use it in the following ways:

1. Display all commands:

   ```
   ./commands
   ```

   or if installed system-wide:

   ```
   commands
   ```

2. Search for a specific command:
   ```
   ./commands -s <keyword>
   ```
   or if installed system-wide:
   ```
   commands -s <keyword>
   ```
   Replace `<keyword>` with the term you want to search for.

## File Structure

- `src/main.c`: The main source code file containing the program logic
- `linux`: A text file containing the list of Linux commands and their descriptions
- `Makefile`: Used for building and managing the project
- `README.md`: This file, containing project documentation
- `LICENSE`: The license file for the project

## How It Works

- The program reads the list of commands from the `linux` file.
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
