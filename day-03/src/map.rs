use crate::slope::Slope;

pub struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(lines: &Vec<Vec<char>>) -> Map {
        return Map {
            map: lines.clone(),
            width: lines.iter().nth(0).unwrap().len(),
            height: lines.len(),
        };
    }

    pub fn trees(&self, slope: &Slope) -> usize {
        let mut x = 0;
        let mut y = 0;

        let mut tree_count = 0;

        while y < self.height {
            if self.map[y][x] == '#' {
                tree_count += 1;
            }

            x = (x + slope.right) % self.width;
            y = y + slope.down;
        }

        return tree_count;
    }
}
