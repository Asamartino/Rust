// Instructions
// Given the size, return a square matrix of numbers in spiral order.

// The matrix should be filled with natural numbers, starting from 1 in the top-left corner, increasing in an inward, clockwise spiral order, like these examples:

// Examples
// Spiral matrix of size 3
// 1 2 3
// 8 9 4
// 7 6 5
// Spiral matrix of size 4
//  1  2  3 4
// 12 13 14 5
// 11 16 15 6
// 10  9  8 7

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![vec![1]];
    }
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let mut count = 1;
    let (mut row_start, mut row_end, mut col_start, mut col_end) =
        (0, size as usize - 1, 0, size as usize - 1);

    while row_start <= row_end && col_start <= col_end {
        // fill top row
        for j in col_start..=col_end {
            matrix[row_start][j] = count;
            count += 1;
        }
        row_start += 1;

        // fill right column
        for i in row_start..=row_end {
            matrix[i][col_end] = count;
            count += 1;
        }
        col_end -= 1;

        // fill bottom row
        if row_start <= row_end {
            for j in (col_start..=col_end).rev() {
                matrix[row_end][j] = count;
                count += 1;
            }
            row_end -= 1;
        }

        // fill left column
        if col_start <= col_end {
            for i in (row_start..=row_end).rev() {
                matrix[i][col_start] = count;
                count += 1;
            }
            col_start += 1;
        }
    }
    matrix
}

fn main() {
    println!("spiral matrix of 3 {:?}", spiral_matrix(3));

    println!("spiral matrix of 4 {:?}", spiral_matrix(4));
}
