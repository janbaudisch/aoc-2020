use common::input;

fn main() {
    let data = input::read_lines()
        .iter()
        .map(|x| u64::from_str_radix(x, 10).unwrap())
        .collect::<Vec<u64>>();

    let invalid = find_invalid(&data, 25);

    println!("[PART ONE] first invalid number: {}", invalid);

    println!(
        "[PART TWO] sum for invalid number: {}",
        find_sum(&data, invalid)
    );
}

fn find_invalid(data: &[u64], preamble: usize) -> u64 {
    for (i, number) in data.iter().skip(preamble).enumerate() {
        let mut flag = false;
        let end = i + preamble;

        'outer: for (a_i, a) in data[i..end].iter().enumerate() {
            for (b_i, b) in data[i..end].iter().enumerate() {
                if a_i != b_i && a + b == *number {
                    flag = true;
                    break 'outer;
                }
            }
        }

        if !flag {
            return *number;
        }
    }

    panic!("Unable to find invalid number!");
}

fn find_sum(data: &[u64], number: u64) -> u64 {
    for (index, _) in data.iter().enumerate() {
        let mut other = data.iter().skip(index);
        let mut sum = Vec::new();

        loop {
            if let Some(next) = other.next() {
                sum.push(*next);
            } else {
                break;
            }

            let current_sum = sum.iter().sum::<u64>();

            if current_sum > number {
                break;
            }

            if current_sum == number {
                sum.sort_unstable();
                return sum[0] + sum.pop().unwrap();
            }
        }
    }

    panic!("Unable to find sum!");
}

#[cfg(test)]
mod tests {
    use super::{find_invalid, find_sum};

    fn generate_data() -> Vec<u64> {
        vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ]
    }

    #[test]
    fn part_one() {
        assert_eq!(find_invalid(&generate_data(), 5), 127);
    }

    #[test]
    fn part_two() {
        let data = generate_data();
        assert_eq!(find_sum(&data, find_invalid(&data, 5)), 62);
    }
}
