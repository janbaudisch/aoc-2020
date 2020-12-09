use common::input;

fn main() {
    let expenses: Vec<u32> = input::read_lines()
        .iter()
        .map(|x| u32::from_str_radix(&x, 10).expect("error converting input"))
        .collect();

    println!(
        "[PART ONE] product of correct expenses: {}",
        calculate_product2(&expenses)
    );

    println!(
        "[PART TWO] product of correct expenses: {}",
        calculate_product3(&expenses)
    );
}

fn calculate_product2(expenses: &[u32]) -> u32 {
    for expense_a in expenses {
        for expense_b in expenses {
            if expense_a != expense_b && expense_a + expense_b == 2020 {
                return expense_a * expense_b;
            }
        }
    }

    0
}

fn calculate_product3(expenses: &[u32]) -> u32 {
    for expense_a in expenses {
        for expense_b in expenses {
            for expense_c in expenses {
                if expense_a != expense_b
                    && expense_a != expense_c
                    && expense_a + expense_b + expense_c == 2020
                {
                    return expense_a * expense_b * expense_c;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::{calculate_product2, calculate_product3};

    #[test]
    fn part_one() {
        assert_eq!(
            calculate_product2(&vec![1721, 979, 366, 299, 675, 1456]),
            514579
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            calculate_product3(&vec![1721, 979, 366, 299, 675, 1456]),
            241861950
        );
    }
}
