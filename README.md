# Ranger

This project is a simple command-line tool written in Rust that helps you create a C/C++ project with a specified build system. It supports three build systems: 

- CMake
- Make 
- Meson.
- Premake5

The tool downloads the necessary template files from this GitHub repository and sets up the project structure for you.

## Features

- Create a C/C++ project with a specified build system.
- Automatically download and configure template files.
- Supports CMake, Make, and Meson build systems.

## Prerequisites

- Rust (1.50 or later)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/thecodefodder/Ranger.git
   cd Ranger
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

To create a new C/C++ project, run the following command:

```bash
cargo run <project_name>
```

Replace `<project_name>` with your desired project name. You will then be prompted to select a build system from the available options:

- CMake
- Make
- Meson
- Premake5

The tool will create a new directory with the specified project name and set up the necessary files based on the selected build system.

## Example

Here’s an example of how to create a new project called `MyProject` using CMake:

```bash
cargo run MyProject
```

After running the command, you will see prompts guiding you through the process. The resulting project structure will look something like this:

```bash
MyProject/
├── CMakeLists.txt
└── src/
    └── main.cpp
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
