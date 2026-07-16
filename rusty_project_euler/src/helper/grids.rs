#[derive(Debug)]
pub struct IntGrid {
    width: u32,
    height: u32,
    data: Vec<u32>,
}

// (0,0) is top right, positive-x is left, positive-y is down
impl IntGrid {
    pub fn get(&self, row: u32, col: u32) -> u32 {
        self.data[(row * self.width + col) as usize]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub fn build_int_grid(width: u32, height: u32, grid_text: &str) -> IntGrid {
    IntGrid {
        width,
        height,
        data: build_grid_data(grid_text),
    }
}

fn build_grid_data(grid_text: &str) -> Vec<u32> {
    let string_data: Vec<&str> = grid_text.split(['\n', ' ']).collect();
    let output: Vec<u32> = string_data
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    output
}
