mod code;
mod instruction;

use code::Code;
use common::input;

fn main() {
    let code: Code = input::read_lines().into();

    let mut part_one = code.clone();
    part_one.run();

    println!(
        "[PART ONE] accumulator before loop: {}",
        part_one.accumulator
    );

    let mut part_two = code;
    part_two.repair();
    part_two.run();

    println!(
        "[PART TWO] accumulator after termintation: {}",
        part_two.accumulator
    );
}

#[cfg(test)]
mod tests {
    use super::Code;

    fn generate_code() -> Code {
        let input = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ];

        input
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .into()
    }

    #[test]
    fn part_one() {
        let mut code = generate_code();
        code.run();
        assert_eq!(code.accumulator, 5);
    }

    #[test]
    fn part_two() {
        let mut code = generate_code();
        code.repair();
        code.run();
        assert_eq!(code.accumulator, 8);
    }
}
