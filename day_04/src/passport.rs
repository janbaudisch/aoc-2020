macro_rules! check_characters {
    ($year:expr, $digits:expr) => {
        if $year.chars().count() != $digits {
            return false;
        }
    };
}

macro_rules! check_number {
    ($number:expr, $min:expr, $max:expr) => {
        if let Ok(number) = u16::from_str_radix($number, 10) {
            if !($min..=$max).contains(&number) {
                return false;
            }
        } else {
            return false;
        }
    };
}

macro_rules! enforce {
    ($validation:expr) => {
        if !$validation {
            return false;
        }
    };
}

pub struct Passport {
    pub birth_year: Option<String>,
    pub issue_year: Option<String>,
    pub expiration_year: Option<String>,
    pub height: Option<String>,
    pub hair_color: Option<String>,
    pub eye_color: Option<String>,
    pub id: Option<String>,
    pub country_id: Option<String>,
}

impl From<&Vec<String>> for Passport {
    fn from(input: &Vec<String>) -> Self {
        let mut passport = Self {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            id: None,
            country_id: None,
        };

        for line in input {
            for item in line.split_whitespace() {
                let splitted: Vec<&str> = item.split(':').collect();
                match splitted[0] {
                    "byr" => passport.birth_year = Some(splitted[1].into()),
                    "iyr" => passport.issue_year = Some(splitted[1].into()),
                    "eyr" => passport.expiration_year = Some(splitted[1].into()),
                    "hgt" => passport.height = Some(splitted[1].into()),
                    "hcl" => passport.hair_color = Some(splitted[1].into()),
                    "ecl" => passport.eye_color = Some(splitted[1].into()),
                    "pid" => passport.id = Some(splitted[1].into()),
                    "cid" => passport.country_id = Some(splitted[1].into()),
                    _ => {}
                }
            }
        }

        passport
    }
}

impl From<Vec<String>> for Passport {
    fn from(input: Vec<String>) -> Self {
        Passport::from(&input)
    }
}

impl Passport {
    pub fn validate_loose(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.id.is_some()
    }

    pub fn validate_strict(&self) -> bool {
        enforce!(self.validate_loose());
        enforce!(validate_birth_year(self.birth_year.as_ref().unwrap()));
        enforce!(validate_issue_year(self.issue_year.as_ref().unwrap()));
        enforce!(validate_expiration_year(
            self.expiration_year.as_ref().unwrap()
        ));
        enforce!(validate_height(self.height.as_ref().unwrap()));
        enforce!(validate_hair_color(self.hair_color.as_ref().unwrap()));
        enforce!(validate_eye_color(self.eye_color.as_ref().unwrap()));
        enforce!(validate_id(self.id.as_ref().unwrap()));
        true
    }
}

fn validate_birth_year(birth_year: &str) -> bool {
    check_characters!(birth_year, 4);
    check_number!(birth_year, 1920, 2002);
    true
}

fn validate_issue_year(issue_year: &str) -> bool {
    check_characters!(issue_year, 4);
    check_number!(issue_year, 2010, 2020);
    true
}

fn validate_expiration_year(expiration_year: &str) -> bool {
    check_characters!(expiration_year, 4);
    check_number!(expiration_year, 2020, 2030);
    true
}

fn validate_height(height: &str) -> bool {
    if height.ends_with("cm") {
        let height = height.replace("cm", "");
        check_number!(&height, 150, 193);
    } else if height.ends_with("in") {
        let height = height.replace("in", "");
        check_number!(&height, 59, 76);
    } else {
        return false;
    }

    true
}

fn validate_hair_color(hair_color: &str) -> bool {
    check_characters!(hair_color, 7);

    if !hair_color.starts_with('#') {
        return false;
    }

    for character in hair_color.chars().skip(1) {
        if !character.is_digit(16) {
            return false;
        }
    }

    true
}

fn validate_eye_color(eye_color: &str) -> bool {
    match eye_color {
        "amb" => {}
        "blu" => {}
        "brn" => {}
        "gry" => {}
        "grn" => {}
        "hzl" => {}
        "oth" => {}
        _ => return false,
    }

    true
}

fn validate_id(id: &str) -> bool {
    check_characters!(id, 9);

    if u32::from_str_radix(id, 10).is_err() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_birth_year() {
        assert!(super::validate_birth_year("2002"));
        assert!(!super::validate_birth_year("2003"));
    }

    #[test]
    fn validate_height() {
        assert!(super::validate_height("60in"));
        assert!(super::validate_height("190cm"));
        assert!(!super::validate_height("190in"));
        assert!(!super::validate_height("190"));
    }

    #[test]
    fn validate_hair_color() {
        assert!(super::validate_hair_color("#123abc"));
        assert!(!super::validate_hair_color("#123abz"));
        assert!(!super::validate_hair_color("123abc"));
    }

    #[test]
    fn validate_eye_color() {
        assert!(super::validate_eye_color("brn"));
        assert!(!super::validate_eye_color("wat"));
    }

    #[test]
    fn validate_id() {
        assert!(super::validate_id("000000001"));
        assert!(!super::validate_id("0123456789"));
    }
}
