pub struct IntegerMatrix {
    grid: Vec< Vec<i32> >
}

impl IntegerMatrix {
    pub fn display(self: &IntegerMatrix) {
        for row in &self.grid {
            for i in row {
                print!("{}", i);
            }
            println!("");
        }
    }

    pub fn width(self: &IntegerMatrix) -> usize {
        self.grid[0].len()
    }

    pub fn len(self: &IntegerMatrix, y: usize) -> usize {
        self.grid[y].len()
    }

    pub fn height(self: &IntegerMatrix) -> usize {
        self.grid.len()
    }

    pub fn get(self: &IntegerMatrix, x: usize, y: usize) -> i32 {
        self.grid[y][x]
    }

    pub fn set(self: &mut IntegerMatrix, x: usize, y: usize, i: i32) {
        self.grid[y][x] = i;
    }
}

pub fn build(width: usize, height: usize) -> IntegerMatrix {
    let mut matrix: IntegerMatrix = IntegerMatrix {
        grid: Vec::new()
    };

    for y in 0..height {
        matrix.grid.push(Vec::new());
        for _ in 0..width {
            matrix.grid[y].push( 0 );
        }
    }

    matrix
}
