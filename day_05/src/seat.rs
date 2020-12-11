#[derive(Clone, PartialEq)]
pub struct Seat {
    row: u8,
    column: u8,
}

impl From<String> for Seat {
    fn from(input: String) -> Self {
        let mut row = input;
        let column = row.split_off(7);

        Seat {
            row: binary_space_partitioning(row, 0, 127, 'F', 'B'),
            column: binary_space_partitioning(column, 0, 7, 'L', 'R'),
        }
    }
}

impl Seat {
    pub fn new(row: u8, column: u8) -> Self {
        Self { row, column }
    }

    pub fn get_id(&self) -> u16 {
        self.row as u16 * 8 + self.column as u16
    }
}

fn binary_space_partitioning(
    input: String,
    lower: u8,
    upper: u8,
    lower_character: char,
    upper_character: char,
) -> u8 {
    let mut lower = lower;
    let mut upper = upper;

    for step in input.chars() {
        let change = ((upper - lower) as f32 / 2_f32).ceil() as u8;

        if step == lower_character {
            upper -= change;
        } else if step == upper_character {
            lower += change;
        } else {
            panic!("Binary space partitioning failed!");
        }
    }

    if lower != upper {
        panic!("Binary space partitioning failed!");
    }

    lower
}
