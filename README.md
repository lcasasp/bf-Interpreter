# Brainf*** Interpreter in Rust

## Overview

This project is a Brainf*** language interpreter written in Rust. The primary goal of this project was to deepen my understanding of Rust, a language renowned for its performance and safety. I decided to tackle Bf in particular due to its simple nature (computationally that is). 

### Installation

Clone the repository:

```
git clone https://github.com/lcasasp/bf-Interpreter.git
cd bf-Interpreter
```

Build the project:

```
cargo build --release
```

### Usage

Run a Brainfuck script:

```
cargo run testing/helloworld.bf
or your own file with:
cargo run /path/../script.bf
```

Or, run the interpreter in interactive mode:

```
cargo run
```

## Development Journey

### Learning Rust

As a sophomore at Cornell University with experience in backend development using Python Flask, I embarked on this project to broaden my programming horizons and learn Rust. Rust's unique approach to memory safety, along with its growing use in both systems and web programming, made it an ideal choice for this endeavor.

### Challenges and Solutions

- **Memory Management**: Implementing the Brainfuck memory tape was a practical exercise in Rust's ownership and borrowing rules.
- **Loop Handling**: The Brainfuck loop commands `[` and `]` posed an interesting challenge, solved by implementing a stack-based approach to track loop positions.
- **I/O Operations**: Developing the `input` and `output` functions helped me understand Rust's approach to I/O streams.

### Key Takeaways

- **Rust's Robustness**: Gained hands-on experience with Rust's compile-time guarantees and pattern matching features.
- **Problem-solving Skills**: Enhanced my problem-solving skills by tackling the complexities of interpreting an esoteric language.
- **Performance Optimization**: Learned about performance optimization in Rust and the importance of efficient memory use.

## Contributing

Contributions to this project are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

Lucas Casas - lucascasasp@icloud.com
