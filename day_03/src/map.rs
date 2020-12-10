#[derive(Clone)]
pub struct Map {
    pub matrix: Vec<Vec<bool>>,
    pub length: usize,
    pub width: usize,
}

impl From<Vec<String>> for Map {
    fn from(input: Vec<String>) -> Self {
        let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(input.len());

        for line in input {
            matrix.push(
                line.chars()
                    .map(|x| {
                        if x == '#' {
                            return true;
                        }

                        false
                    })
                    .collect::<Vec<bool>>(),
            );
        }

        let length = &matrix.len();
        let width = &matrix[0].len();

        Self {
            matrix,
            length: *length,
            width: *width,
        }
    }
}
