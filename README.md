[![Build](https://github.com/emersonmde/atium/actions/workflows/rust.yml/badge.svg)](https://github.com/emersonmde/atium/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

# Atium - A Computer Algebra System (CAS)

Atium is a Computer Algebra System (CAS) written in Rust, designed to parse, simplify, and visually represent algebraic expressions. It leverages Typst for typesetting and iTerm2's advanced capabilities for inline image display.

## Features

### Currently Supported

- Parsing basic arithmetic expressions involving addition (`+`) and multiplication (`*`).
- Simplification of expressions including flattening nested structures and combining like terms.
- Generating representations of expressions in Typist format.
- Visual output as PNG images using iTerm2's `imgcat` for users on macOS.

### In Progress

- Extension to support variables and algebraic expressions.
- Advanced expression manipulation techniques like distribution and factoring.
- Introduction of calculus features such as derivatives and integrals.
- Capabilities to solve equations and systems of equations.
- Graphical plotting of functions and expressions.

## Prerequisites

- **iTerm2**: A terminal emulator for macOS that supports image display.
- **iTerm2 Shell Integration**: Provides features like image display directly in the terminal.
- **Typst**: A utility for rendering mathematical expressions, used by Atium for visual output.

### Installation Guide

1. **iTerm2 Installation (macOS)**

   Download and install iTerm2 from [here](https://iterm2.com/).

2. **iTerm2 Shell Integration**

   Install the iTerm2 shell integrations by executing the following in iTerm2:

   ```bash
   curl -L https://iterm2.com/shell_integration/install_shell_integration_and_utilities.sh | bash
   ```

3. **Typst Installation**

   Use Homebrew to install Typst:

   ```bash
   brew install typst
   ```

## Usage

With the prerequisites in place, here's how you can get started with Atium:

1. **Clone Atium Repository**

   ```sh
   git clone https://github.com/emersonmde/atium.git && cd atium
   ```

2. **Run Atium**

   Execute the CAS with an example expression:

   ```sh
   cargo run --release -- "3+1*2*3*4+5*x"
   ```

   The console will display the original and simplified expressions, and iTerm2 users will see the visual representation.

## Development Status

Atium is actively being developed. While it's not yet feature-complete, the foundations for expression parsing and simplification are in place, and visual output via Typist is supported.

Feel free to explore the project and test its current capabilities

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.