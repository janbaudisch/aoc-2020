mod passport;

use common::input;
use passport::Passport;

fn main() {
    let passports: Vec<Passport> = input::read_blocks().iter().map(|x| x.into()).collect();

    let validated = passports
        .iter()
        .map(|x| x.validate_loose())
        .map(|x| if x { 1 } else { 0 });

    println!("[PART ONE] valid passports: {}", validated.sum::<u32>());

    let validated = passports
        .iter()
        .map(|x| x.validate_strict())
        .map(|x| if x { 1 } else { 0 });

    println!("[PART TWO] valid passports: {}", validated.sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::Passport;

    fn generate_input1() -> Vec<Vec<&'static str>> {
        vec![
            vec![
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
                "byr:1937 iyr:2017 cid:147 hgt:183cm",
            ],
            vec![
                "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
                "hcl:#cfa07d byr:1929",
            ],
            vec![
                "hcl:#ae17e1 iyr:2013",
                "eyr:2024",
                "ecl:brn pid:760753108 byr:1931",
                "hgt:179cm",
            ],
            vec![
                "hcl:#cfa07d eyr:2025 pid:166559648",
                "iyr:2011 ecl:brn hgt:59in",
            ],
        ]
    }

    fn generate_input2() -> Vec<Vec<&'static str>> {
        vec![
            vec![
                "eyr:1972 cid:100",
                "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            ],
            vec![
                "iyr:2019",
                "hcl:#602927 eyr:1967 hgt:170cm",
                "ecl:grn pid:012533040 byr:1946",
            ],
            vec![
                "hcl:dab227 iyr:2012",
                "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            ],
            vec![
                "hgt:59cm ecl:zzz",
                "eyr:2038 hcl:74454a iyr:2023",
                "pid:3556412378 byr:2007",
            ],
        ]
    }

    fn generate_input3() -> Vec<Vec<&'static str>> {
        vec![
            vec![
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
                "hcl:#623a2f",
            ],
            vec![
                "eyr:2029 ecl:blu cid:129 byr:1989",
                "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            ],
            vec![
                "hcl:#888785",
                "hgt:164cm byr:2001 iyr:2015 cid:88",
                "pid:545766238 ecl:hzl",
                "eyr:2022",
            ],
            vec!["iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"],
        ]
    }

    fn parse_input(input: Vec<Vec<&str>>) -> Vec<Passport> {
        input
            .iter()
            .map(|x| {
                x.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .into()
            })
            .collect()
    }

    #[test]
    fn part_one() {
        assert_eq!(
            parse_input(generate_input1())
                .iter()
                .map(|x| x.validate_loose())
                .map(|x| if x { 1 } else { 0 })
                .sum::<u8>(),
            2
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            parse_input(generate_input1())
                .iter()
                .map(|x| x.validate_strict())
                .map(|x| if x { 1 } else { 0 })
                .sum::<u8>(),
            2
        );

        assert_eq!(
            parse_input(generate_input2())
                .iter()
                .map(|x| x.validate_strict())
                .map(|x| if x { 1 } else { 0 })
                .sum::<u8>(),
            0
        );

        assert_eq!(
            parse_input(generate_input3())
                .iter()
                .map(|x| x.validate_strict())
                .map(|x| if x { 1 } else { 0 })
                .sum::<u8>(),
            4
        );
    }
}
