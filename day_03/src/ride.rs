use crate::Map;

pub struct Ride {
    map: Map,
    change_x: usize,
    change_y: usize,
    x: usize,
    y: usize,
}

impl Iterator for Ride {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        self.x += self.change_x;
        self.y += self.change_y;

        if self.y >= self.map.length {
            return None;
        }

        if self.x >= self.map.width {
            self.x -= self.map.width;
        }

        Some(self.map.matrix[self.y][self.x])
    }
}

impl Ride {
    pub fn create(map: Map, change_x: usize, change_y: usize) -> Self {
        Self {
            map,
            change_x,
            change_y,
            x: 0,
            y: 0,
        }
    }

    pub fn ride(self) -> u32 {
        self.map(|x| if x { 1 } else { 0 }).sum()
    }
}
