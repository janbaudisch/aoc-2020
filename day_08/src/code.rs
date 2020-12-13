use crate::instruction::Instruction;

#[derive(Clone, Debug)]
pub struct Code {
    instructions: Vec<Instruction>,
    pointer: usize,
    visited: Vec<usize>,
    pub accumulator: i32,
}

impl From<Vec<String>> for Code {
    fn from(input: Vec<String>) -> Self {
        let instructions = input.iter().map(|x| x.into()).collect();

        Self {
            instructions,
            pointer: 0,
            visited: Vec::new(),
            accumulator: 0,
        }
    }
}

impl Code {
    fn step(&mut self) {
        self.visited.push(self.pointer);

        let mut pointer = self.pointer as isize;

        match self.instructions[self.pointer] {
            Instruction::Accumulate(i) => self.accumulator += i,
            Instruction::Jump(i) => pointer += i as isize - 1,
            Instruction::Noop(_) => {}
        }

        self.pointer = pointer as usize + 1;
    }

    pub fn run(&mut self) {
        while !self.visited.contains(&self.pointer) && self.pointer < self.instructions.len() {
            self.step();
        }
    }

    pub fn repair(&mut self) {
        let mut test_pointer = self.instructions.len();

        loop {
            test_pointer -= 1;

            let mut test = self.clone();

            match test.instructions[test_pointer] {
                Instruction::Jump(i) => test.instructions[test_pointer] = Instruction::Noop(i),
                Instruction::Noop(i) => test.instructions[test_pointer] = Instruction::Jump(i),
                _ => {}
            }

            test.run();

            if test.pointer == test.instructions.len() {
                self.instructions = test.instructions;
                return;
            }
        }
    }
}
