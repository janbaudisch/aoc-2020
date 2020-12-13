#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Accumulate(i32),
    Jump(i32),
    Noop(i32),
}

impl From<&String> for Instruction {
    fn from(input: &String) -> Self {
        let mut parts = input.split_whitespace();

        match parts.next().unwrap() {
            "acc" => Self::Accumulate(i32::from_str_radix(parts.next().unwrap(), 10).unwrap()),
            "jmp" => Self::Jump(i32::from_str_radix(parts.next().unwrap(), 10).unwrap()),
            "nop" => Self::Noop(i32::from_str_radix(parts.next().unwrap(), 10).unwrap()),
            _ => panic!("error converting input"),
        }
    }
}
