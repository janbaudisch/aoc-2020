use std::collections::HashMap;

#[derive(Clone)]
pub struct Rule {
    pub color: String,
    pub contents: HashMap<String, u32>,
}

impl From<&str> for Rule {
    fn from(input: &str) -> Self {
        let mut words = input.split_whitespace();

        let mut color = words.next().unwrap().to_string();
        color.push(' ');
        color.push_str(words.next().unwrap());

        let _ = words.next();
        let _ = words.next();

        let mut contents = HashMap::new();

        while let Some(count) = words.next() {
            if count == "no" {
                break;
            }

            let count = u32::from_str_radix(count, 10).unwrap();

            let mut color = words.next().unwrap().to_string();
            color.push(' ');
            color.push_str(words.next().unwrap());

            contents.insert(color, count);

            let _ = words.next();
        }

        Self { color, contents }
    }
}

impl From<&String> for Rule {
    fn from(input: &String) -> Self {
        Rule::from(input.as_str())
    }
}
