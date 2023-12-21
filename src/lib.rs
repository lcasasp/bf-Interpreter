//Bf uses an array of 30000 1 byte memory blocks
const TAPE_SIZE: usize = 30000;

pub struct BfInterpreter {
    // The memory tape
    tape: [u8; TAPE_SIZE],
    // The data pointer
    pointer: usize,
}

impl BfInterpreter {
    pub fn new() -> Self {
        BfInterpreter {
            tape: [0; TAPE_SIZE],
            pointer: 0
        }
    }

    pub fn execute(&mut self, program: &str) {
        let mut program_counter = 0;
        let program_length = program.len();
        let program_chars: Vec<char> = program.chars().collect();

        while program_counter < program_length {
            match program_chars[program_counter] {
                '>' => self.move_right(),
                '<' => self.move_left(),
                '+' => self.increment(),
                '-' => self.decrement(),
                '.' => self.output(),
                ',' => self.input(),
                '[' => {
                    if self.tape[self.pointer] == 0 {
                        program_counter = self.jump_forward(program_counter, &program_chars);
                        continue;
                    }
                }
                ']' => {
                    if self.tape[self.pointer] != 0 {
                        program_counter = self.jump_backward(program_counter, &program_chars);
                        continue;
                    }
                }
                _ => {} // non-command characters are ignored as comments
            }
            program_counter += 1;
        }
    }

    pub fn reset(&mut self) {
        self.tape = [0; TAPE_SIZE];
        self.pointer = 0;
    }
}

impl BfInterpreter {
    //handle the '>' command
    fn move_right(&mut self) {
        self.pointer += 1;
    }

    //handle the '<' command
    fn move_left(&mut self) {
        self.pointer -= 1;
    }

    //handle the '+' command
    fn increment(&mut self) {
        if self.pointer < TAPE_SIZE {
            self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1);
        } else {
            panic!("Pointer out of bounds");
        }
    }

    //handle the '-' command
    fn decrement(&mut self) {
        if self.pointer < TAPE_SIZE {
            self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1);
        } else {
            panic!("Pointer out of bounds");
        }
    }

    //handle the '.' command
    fn output(&self) {
        use std::io::{self, Write};

        let output_char = self.tape[self.pointer] as char;
        print!("{}", output_char);
        io::stdout().flush().unwrap()
    }

    //handle the ',' command
    fn input(&mut self) {
        use std::io::{self, Read};

        let mut input_byte: [u8; 1] = [0; 1];
        if io::stdin().read_exact(&mut input_byte).is_ok() {
            self.tape[self.pointer] = input_byte[0];
        } else {
            panic!("Error reading input");
        }
    }

    // handle '[' and ']' commands for loops
    fn jump_forward(&self, start: usize, program: &[char]) -> usize {
        let mut counter = 1;
        let mut i = start + 1;

        while i < program.len() && counter > 0 {
            match program[i] {
                '[' => counter += 1,
                ']' => counter -= 1,
                _ => {}
            }
            i += 1;
        }

        if counter == 0 {
            i 
        } else {
            start // Fallback: return original position
        }
    }

    fn jump_backward(&self, start: usize, program: &[char]) -> usize {
        if start == 0 {
            return 0;
        }

        let mut counter = 1;
        let mut i = start;

        while i > 0 {
            i -= 1;
            match program[i] {
                '[' if counter == 1 => return i + 1,
                '[' => counter -= 1,
                ']' => counter += 1,
                _ => {}
            }
        }
        start  // Fallback: return original position
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let interpreter: BfInterpreter = BfInterpreter::new();

        // Check cells initialized to 0
        assert!(interpreter.tape.iter().all(|&cell| cell == 0), "Memory tape should be initialized with zeros");

        // Check data pointer at pos 0
        assert_eq!(interpreter.pointer, 0, "Data pointer should be initialized to 0");
    }

    #[test]
    fn test_increment() {
        let mut interpreter: BfInterpreter = BfInterpreter::new();
        
        //test incrementing
        interpreter.increment();
        assert_eq!(interpreter.tape[interpreter.pointer], 1);

        //test wrapping
        interpreter.tape[interpreter.pointer] = 255;
        interpreter.increment();
        assert_eq!(interpreter.tape[interpreter.pointer], 0);
    }

    #[test]
    fn test_decrement() {
        let mut interpreter: BfInterpreter = BfInterpreter::new();
        
        //test decrementing
        interpreter.tape[interpreter.pointer] = 1;
        interpreter.decrement();
        assert_eq!(interpreter.tape[interpreter.pointer], 0);

        //test wrapping
        interpreter.tape[interpreter.pointer] = 0;
        interpreter.decrement();
        assert_eq!(interpreter.tape[interpreter.pointer], 255);
    }

    #[test]
    fn test_jump_forward() {
        let interpreter: BfInterpreter = BfInterpreter::new();
        let program: Vec<char> = vec!['[', '+', '-', ']'];
        let jump_position: usize = interpreter.jump_forward(0, &program);

        assert_eq!(jump_position, 4, "Should jump to the position after matching ']'");
    }

    #[test]
    fn test_jump_backward() {
        let interpreter: BfInterpreter = BfInterpreter::new();
        let program: Vec<char> = vec!['[', '+', '-', ']'];
        let jump_position: usize = interpreter.jump_backward(3, &program);

        assert_eq!(jump_position, 1, "Should jump to the position after matching '['");
    }
}
