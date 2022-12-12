pub struct Matrix<T> {
    grid: Vec< Vec<T> >
}

impl<T: std::fmt::Display + Clone> Matrix<T> {
    pub fn display(self: &Matrix<T>) {
        for row in &self.grid {
            for item in row {
                print!("{}", item);
            }
            println!("");
        }
    }

    pub fn numeric_display(self: &Matrix<T>) {
        for row in &self.grid {
            for item in row {
                print!(" {:4}", item);
            }
            println!("");
        }
    }

    pub fn width(self: &Matrix<T>) -> usize {
        self.grid[0].len()
    }

    pub fn len(self: &Matrix<T>, y: usize) -> usize {
        self.grid[y].len()
    }

    pub fn height(self: &Matrix<T>) -> usize {
        self.grid.len()
    }

    pub fn get(self: &Matrix<T>, x: usize, y: usize) -> T {
        self.grid[y][x].clone()
    }

    pub fn set(self: &mut Matrix<T>, x: usize, y: usize, item: T) {
        self.grid[y][x] = item;
    }
}

pub fn load_char_matrix(lines: &Vec<String>) -> Matrix<char> {
    let mut matrix: Matrix<char> = Matrix { grid: Vec::new() };

    for line in lines {
        matrix.grid.push( line.chars().collect() );
    }

    matrix
}

pub fn build<T>(width: usize, height: usize, default: T) -> Matrix<T> where T: Clone + Copy {
    let mut matrix: Matrix<T> = Matrix { grid: Vec::new() };

    for y in 0..height {
        matrix.grid.push(Vec::new());
        matrix.grid[y].resize(width, default);
    }

    matrix
}
