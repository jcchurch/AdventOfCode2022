pub struct CharMatrix {
    grid: Vec< Vec<char> >
}

impl CharMatrix {
    pub fn display(self: &CharMatrix) {
        for row in &self.grid {
            for ch in row {
                print!("{}", ch);
            }
            println!("");
        }
    }

    pub fn width(self: &CharMatrix) -> usize {
        self.grid[0].len()
    }

    pub fn len(self: &CharMatrix, y: usize) -> usize {
        self.grid[y].len()
    }

    pub fn height(self: &CharMatrix) -> usize {
        self.grid.len()
    }

    pub fn get(self: &CharMatrix, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    pub fn set(self: &mut CharMatrix, x: usize, y: usize, ch: char) {
        self.grid[y][x] = ch;
    }
}

pub fn build(lines: &Vec<String>) -> CharMatrix {
    let mut matrix: CharMatrix = CharMatrix {
        grid: Vec::new()
    };

    for line in lines {
        matrix.grid.push( line.chars().collect() );
    }

    matrix
}
