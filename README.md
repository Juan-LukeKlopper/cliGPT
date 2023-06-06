# cliGPT

This project is a simple cli tool to inteact with chatGPT written in rust.

# Getting Started

To use this project, make sure you have Rust and Cargo installed. If you don't have them installed, you can install them using rustup.

# Installing rustup

rustup is the official Rust toolchain installer. You can download and install it by following the instructions on the official Rust website .

# Setting up the project

Clone this repository to your local machine:

```git clone https://github.com/Juan-LukeKlopper/cliGPT.git```

Change to the project directory:

```cd cliGPT```

Set your OpenAI API Key

```export OPENAI_API_KEY="sk-YOURAPIKEYHERE"```

Compile the code using Cargo:

```cargo build```

Cargo will take care of fetching the required dependencies, and compiling the code.

Move the program to a executable path

```cp target/debug/cliGPT /usr/local/bin/```

To Execute the program run

```cliGPT --prompt "Enter prompt here"```
