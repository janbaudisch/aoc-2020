pub struct Group(Vec<Vec<char>>);

impl From<&Vec<String>> for Group {
    fn from(input: &Vec<String>) -> Self {
        let mut persons = Vec::with_capacity(input.len());

        for person in input {
            persons.push(person.chars().collect());
        }

        Self(persons)
    }
}

impl From<Vec<String>> for Group {
    fn from(input: Vec<String>) -> Self {
        Group::from(&input)
    }
}

impl Group {
    pub fn count_anyone_yes(&self) -> usize {
        let mut combined = self.0.concat();
        combined.sort_unstable();
        combined.dedup();
        combined.len()
    }

    pub fn count_everyone_yes(&self) -> usize {
        let first = &self.0[0];

        if self.0.len() == 1 {
            return first.len();
        }

        let mut count = 0;

        'answers: for answer in first {
            for other in self.0.iter() {
                if !other.contains(answer) {
                    continue 'answers;
                }
            }

            count += 1;
        }

        count
    }
}
