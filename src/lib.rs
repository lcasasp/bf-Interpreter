use std::collections::HashMap;

//Bf uses an array of 30000 1 byte memory blocks
const TAPE_SIZE: usize = 30000;

pub struct BfInterpreter {
    // The memory tape
    tape: [u8; TAPE_SIZE],
    // The data pointer
    pointer: usize,
    // Other fields as needed (e.g., for loop handling)
}

impl BfInterpreter {
    pub fn new() -> Self {
        BfInterpreter {
            tape: [0; TAPE_SIZE],
            pointer: 0
        }
    }

    pub fn execute(&mut self, program: &str) {
       1;
    }

    pub fn reset(&mut self) {
        self.tape = [0; TAPE_SIZE];
        self.pointer = 0;
    }
}

impl BfInterpreter {
    // Function to handle the '>' command
    fn move_right(&mut self) {
        self.pointer += 1;
    }

    // Function to handle the '<' command
    fn move_left(&mut self) {
        self.pointer -= 1;
    }

    // Function to handle the '+' command
    fn increment(&mut self) {
        if self.pointer < TAPE_SIZE {
            self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1);
        } else {
            panic!("Pointer out of bounds");
        }
    }

    // Function to handle the '-' command
    fn decrement(&mut self) {
        if self.pointer < TAPE_SIZE {
            self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1);
        } else {
            panic!("Pointer out of bounds");
        }
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
    fn test_deccrement() {
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
}
