# Commands

```
 ██████╗ ██████╗ ███╗   ███╗███╗   ███╗ █████╗ ███╗   ██╗██████╗ ███████╗
██╔════╝██╔═══██╗████╗ ████║████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝
██║     ██║   ██║██╔████╔██║██╔████╔██║███████║██╔██╗ ██║██║  ██║███████╗
██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██╔══██║██║╚██╗██║██║  ██║╚════██║
╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║██████╔╝███████║
 ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝
```

Commands is a simple command-line tool that lists and searches available Linux
commands directly in your terminal. It's designed to help users quickly discover
and explore the commands they can use in their Linux environment.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Customization](#customization)
- [Contributing](#contributing)
- [License](#license)

## Features

- Displays a comprehensive list of all available Linux commands
- Simple and intuitive command-line interface
- Supports searching for specific commands
- Gradual display of commands for better readability

## Installation

### Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)

### Building from Source

1. Clone the repository:

   ```
   git clone https://github.com/williammuchui/linux-command-search.git
   ```

2. Navigate to the project directory:

   ```
   cd linux-command-search
   ```

3. Build the project:

   ```
   cargo build --release
   ```

4. The binary will be available in `target/release/commands`

## Usage

To use the Commands tool:

1. List all available commands:

   ```bash
   commands
   ```

   or

   ```bash
   commands -l
   ```

   or

   ```bash
   commands --list
   ```

2. Search for a specific command:

   ```bash
   commands -s <search_term>
   ```

   or

   ```bash
   commands --search <search_term>
   ```

3. Display help information:
   ```bash
   commands -h
   ```
   or
   ```bash
   commands --help
   ```

### Example Output

```bash
Available Linux Commands:
1. ls - List directory contents
2. cd - Change directory
3. cp - Copy files and directories
...
```

## Customization

You can easily add or modify commands by editing the `linux` file in the project root. Each line should follow this format:

```
command_name - Command description
```

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/AmazingFeature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
5. Push to the branch (`git push origin feature/AmazingFeature`)
6. Open a Pull Request

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

