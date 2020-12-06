pub struct Combinations {
    elements: Vec<u32>,
    group_size: usize,
    max_indices: Vec<usize>,
    current_indices: Vec<usize>,
}

impl Combinations {
    pub fn new(elements: Vec<u32>, group_size: usize) -> Combinations {
        // start out with eg: [0, 1, 1]
        // so the first incrementing iteration will produce [0, 1, 2]
        // which is the first set of indexes we want
        // to use to generate the first combination
        let mut current_indices: Vec<usize> = (0..(group_size - 1)).collect();
        current_indices.push(group_size - 2);

        let num_elements = elements.len();
        let max_indices = ((num_elements - group_size)..num_elements).collect();

        return Combinations {
            elements,
            group_size,
            max_indices,
            current_indices,
        };
    }
}

impl Iterator for Combinations {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_indices == self.max_indices {
            return None;
        }

        for i in (0..self.group_size).rev() {
            if self.current_indices[i] < self.max_indices[i] {
                self.current_indices[i] += 1;
                if i < self.group_size - 1 {
                    for j in (i + 1)..(self.group_size) {
                        self.current_indices[j] = self.current_indices[i] + (j - i);
                    }
                }
                break;
            }
        }

        let combination: Vec<u32> = self
            .current_indices
            .iter()
            .map(|&i| self.elements[i])
            .collect();

        return Some(combination);
    }
}
