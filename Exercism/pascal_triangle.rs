// Instructions
// Compute Pascal's triangle up to a given number of rows.

// In Pascal's Triangle each number is computed by adding the numbers to the right and left of the current position in the previous row.

//     1
//    1 1
//   1 2 1
//  1 3 3 1
// 1 4 6 4 1
// # ... etc

#[derive(Debug)]
pub struct PascalsTriangle {
    data: Vec<Vec<u32>>, // as don't know the size at compile time, needs to be a collection
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut data: Vec<Vec<u32>> = Vec::new();
        for row in 0..row_count as usize {
            let mut final_row = Vec::new();
            for column in 0..=row {
                let value = if column == 0 || column == row {
                    1
                } else {
                    data[(row - 1)][(column - 1)] + data[(row - 1)][column]
                };
                final_row.push(value);
            }
            data.push(final_row);
        }
        PascalsTriangle { data }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.data.clone()
    }
}

fn main() {
    let abc = PascalsTriangle::new(5);
    println!("pascal_triangle {:?}", abc);
}
