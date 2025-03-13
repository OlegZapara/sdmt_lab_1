# Quadratic Equation Solver

This application solves quadratic equations of the form *ax² + bx + c = 0*.

## Build the application

### Prerequisites
Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.

### Clone the Repository
```bash
git clone https://github.com/vitaliichyhryn/sdmt_lab_1.git
cd sdmt_lab_1
```

### Build the Application
```bash
cargo build --release
```

## Run the application

### Interactive Mode (Default)
Run the application without any arguments to enter interactive mode:
```bash
cargo run --release
```

### Non-Interactive Mode
Provide a file containing the coefficients as a command-line argument:
```bash
cargo run --release coefficients.txt
```

#### Non-Interactive Mode File Format
The file should contain the coefficients *a*, *b*, and *c* on a single line, separated by spaces. For example:
```
1.0 -3.0 2.0
```
This represents the quadratic equation: *1.0 \* x ^ 2 - 3.0 \* x + 2.0 = 0*.

## Revert commit
As per the instructions, this repository contains a [revert commit](https://github.com/vitaliichyhryn/sdmt_lab_1/commit/f6f928ea061453ba9f4b3326876320513a6c33ab).
