# Commands

```
 ██████╗ ██████╗ ███╗   ███╗███╗   ███╗ █████╗ ███╗   ██╗██████╗ ███████╗
██╔════╝██╔═══██╗████╗ ████║████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝
██║     ██║   ██║██╔████╔██║██╔████╔██║███████║██╔██╗ ██║██║  ██║███████╗
██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██╔══██║██║╚██╗██║██║  ██║╚════██║
╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║██████╔╝███████║
 ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝
```

Commands is a simple command-line tool that lists all available Linux commands
directly in your terminal. It's designed to help users quickly discover and
explore the commands they can use in their Linux environment.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Customization](#customization)
- [Contributing](#contributing)
- [License](#license)

## Features

Displays a comprehensive list of all available Linux commands.
Simple and intuitive command-line interface.
Supports filtering and searching for specific commands (future enhancement).

### Downloading the Source Code

1. Clone the repository:

   ```
   git clone https://github.com/williammuchui/linux-command-search.git
   ```

2. Navigate to the project directory:
   ```
   cd linux-command-search
   ```

## Installation

I'm yet to implement this section

## Usage

To use the commands tool, simply run:

To List all available commands

```bash
commands
```

OR

```bash
commands -l
```

This will display a list of all available Linux commands in your terminal.  
Example Output

```bash
Available Linux Commands:
1. ls - List directory contents
2. cd - Change directory
3. cp - Copy files and directories
```

## Customization

You can easily add or modify commands by editing the `linux` file. Each line should follow this format:

[command_name] Command description

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features,
please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](license) file for more details.
