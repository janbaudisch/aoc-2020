mod group;

use common::input;
use group::Group;

fn main() {
    let groups: Vec<Group> = input::read_blocks().iter().map(|x| x.into()).collect();

    let count: u32 = groups.iter().map(|x| x.count_anyone_yes() as u32).sum();

    println!("[PART ONE] sum of \"yes\" answeres from anyone: {}", count);

    let count: u32 = groups.iter().map(|x| x.count_everyone_yes() as u32).sum();

    println!(
        "[PART TWO] sum of \"yes\" answeres from everyone: {}",
        count
    );
}

#[cfg(test)]
mod tests {
    use super::Group;

    fn generate_groups() -> Vec<Group> {
        let input = vec![
            vec!["abc"],
            vec!["a", "b", "c"],
            vec!["ab", "ac"],
            vec!["a", "a", "a", "a"],
            vec!["b"],
        ];

        input
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            })
            .map(|x| x.into())
            .collect()
    }

    #[test]
    fn part_one() {
        assert_eq!(
            generate_groups()
                .iter()
                .map(|x| x.count_anyone_yes() as u32)
                .sum::<u32>(),
            11
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            generate_groups()
                .iter()
                .map(|x| x.count_everyone_yes() as u32)
                .sum::<u32>(),
            6
        );
    }
}
