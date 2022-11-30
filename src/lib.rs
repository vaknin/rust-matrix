use std::fmt;
type MatrixResult<Matrix> = std::result::Result<Matrix, MatrixErrors>;

#[derive(Debug)]
pub enum MatrixErrors {
    MismatchedOrders,
    InvalidMatrix
}

#[derive(PartialEq)]
struct Order {
    rows: usize,
    columns: usize
}

pub struct Matrix {
    vectors: Vec<Vec<f32>>,
    order: Order
}
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matrix_as_string = String::new();
        for vector in &self.vectors {
            let vector_as_string = "|".to_string() + &vector.iter().map(|x| x.to_string() + " ").collect::<String>() + "|\n";
            matrix_as_string.push_str(&vector_as_string);
        }
        write!(f, "{}", matrix_as_string)
    }
}
impl Matrix {
    pub fn new (vectors: &Vec<Vec<f32>>) -> MatrixResult<Matrix> {
        
        // Columns
        let mut columns: Option<usize> = None;
        for row in vectors.iter() {
            match columns {
                Some(val) => {
                    if val != row.len() {
                        return Err(MatrixErrors::InvalidMatrix);
                    }
                },
                None => columns = Some(row.len())
            }
        }

        let rows = vectors.len();
        let columns =
            if rows == 0 {
                0
            } else {
            vectors[0].len()
        };

        Ok(Matrix {
            vectors: vectors.to_vec(),
            order: Order { rows, columns }
        })
    }

    pub fn add (&self, other: &Matrix) -> MatrixResult<Matrix> {

        // Check for mismatched orders
        if self.order != other.order {
            return Err(MatrixErrors::MismatchedOrders);
        }

        let mut matrix_vectors: Vec<Vec<f32>> = Vec::new();

        for (i, row) in self.vectors.iter().enumerate() {
            let mut row_vector = Vec::new();
            for j in 0..row.len() {
                let ij = self.vectors[i][j] + other.vectors[i][j];
                row_vector.push(ij);
            }       
            matrix_vectors.push(row_vector);
        }

        Matrix::new(&matrix_vectors)
    }

    pub fn transpose(&self) -> MatrixResult<Matrix> {
        let mut matrix_vectors: Vec<Vec<f32>> = Vec::new();
        for c in 0..self.order.columns {
            let mut new_row: Vec<f32> = Vec::new();
            for r in 0..self.order.rows {
                new_row.push(self.vectors[r][c]);
            }
            matrix_vectors.push(new_row);
        }
        Matrix::new(&matrix_vectors)
    }

    pub fn mult(&self, other: &Matrix) -> MatrixResult<Matrix> {

        // Check for mismatched orders
        if self.order.columns != other.order.rows {
            return Err(MatrixErrors::MismatchedOrders);
        }

        let mut matrix_vectors: Vec<Vec<f32>> = Vec::new();

        // Iterate over self rows
        for row in self.vectors.iter() {

            let mut new_row_vector: Vec<f32> = Vec::new();

            // Other's columns
            for c in 0..other.order.columns {
                
                // Build a column vector
                let mut col_vector: Vec<f32> = Vec::new();
                for r in 0..other.order.rows {
                    col_vector.push(other.vectors[r][c]);
                }

                // Scalar multiplication
                let sum = scalar_mult(row, &col_vector);
                new_row_vector.push(sum);
            }

            // Add the row to the new vector
            matrix_vectors.push(new_row_vector);
        }

        Matrix::new(&matrix_vectors)
    }
}

fn scalar_mult(row: &Vec<f32>, col: &Vec<f32>) -> f32 {

    let mut sum: f32 = 0.0;
    for i in 0..row.len() {
        let mult = row[i] * col[i];
        sum += mult;
    }
    sum
}