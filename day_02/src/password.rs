#[derive(Clone)]
pub struct Password {
    a: usize,
    b: usize,
    character: char,
    password: String
}

impl From<String> for Password {
    fn from(input: String) -> Self {
        let splitted: Vec<&str> = input.split(": ").collect();
        let policy: Vec<&str> = splitted[0].split_whitespace().collect();
        let a_b: Vec<usize> = policy[0].split("-").map(|x| usize::from_str_radix(x, 10).expect("error converting input")).collect();

        Password {
            a: a_b[0],
            b: a_b[1],
            character: policy[1].chars().next().expect("error converting input"),
            password: splitted[1].to_string()
        }
    }
}

impl Password {
    pub fn check_sled_rental(&self) -> bool {
        let count: usize = self.password.chars().map(|character| {
            if character == self.character {
                return 1;
            } else {
                return 0;
            }
        }).sum();

        count >= self.a && count <= self.b
    }

    pub fn check_toboggan_corporate(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();

        (chars[self.a - 1] == self.character) ^ (chars[self.b - 1] == self.character)
    }
}
