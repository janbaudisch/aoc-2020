mod rule;

use common::input;
use rule::Rule;

fn main() {
    let rules: Vec<Rule> = input::read_lines().iter().map(|x| x.into()).collect();

    let count: u32 = rules
        .iter()
        .map(|x| can_contain(x, &rules, "shiny gold") as u32)
        .sum();

    println!("[PART ONE] possible number of outer bags: {}", count);

    let count = must_contain(&rules, "shiny gold");

    println!("[PART TWO] number of bags: {}", count);
}

fn can_contain(rule: &Rule, rules: &[Rule], color: &str) -> bool {
    if rule.contents.contains_key(color) {
        return true;
    }

    for (rule_color, _) in rule.clone().contents {
        if can_contain(
            rules.iter().find(|x| x.color == rule_color).unwrap(),
            rules,
            &color,
        ) {
            return true;
        }
    }

    false
}

fn must_contain(rules: &[Rule], color: &str) -> u32 {
    let mut count = 0;

    let rule = rules.iter().find(|x| x.color == color).unwrap();

    for (rule_color, rule_count) in rule.clone().contents {
        count += rule_count;
        count += must_contain(rules, &rule_color) * rule_count;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::{can_contain, must_contain, Rule};

    fn generate_input1() -> Vec<&'static str> {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        input.into_iter().map(|x| x.into()).collect()
    }

    fn generate_input2() -> Vec<&'static str> {
        let input = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];

        input.into_iter().map(|x| x.into()).collect()
    }

    fn parse_rules(input: Vec<&str>) -> Vec<Rule> {
        input.into_iter().map(|x| x.into()).collect()
    }

    #[test]
    fn part_one() {
        let rules = parse_rules(generate_input1());
        assert_eq!(
            rules
                .iter()
                .map(|x| can_contain(x, &rules, "shiny gold") as u32)
                .sum::<u32>(),
            4
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            must_contain(&parse_rules(generate_input1()), "shiny gold"),
            32
        );

        assert_eq!(
            must_contain(&parse_rules(generate_input2()), "shiny gold"),
            126
        );
    }
}
