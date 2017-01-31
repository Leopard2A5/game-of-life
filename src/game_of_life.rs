#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Cell {
    Live,
    Dead,
}

impl Cell {
    pub fn to_char(&self) -> char {
        match *self {
            Cell::Dead => ' ',
            Cell::Live => 'X',
        }
    }
}

pub struct GameOfLife {
    cells: Vec<Vec<Cell>>
}

impl GameOfLife {
    pub fn new(
        width: usize,
        height: usize,
    ) -> Self {
        GameOfLife {
            cells: vec![vec![Cell::Dead; width]; height],
        }
    }

    pub fn step(&mut self) {
        let mut new_field = self.cells.clone();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cell = self.cells[y][x];
                let neighbors = self.get_neighbors(x, y);
                let num_live_neighbors = neighbors.iter()
                    .filter(|x| **x == Cell::Live )
                    .count();
                match cell {
                    Cell::Live => {
                        new_field[y][x] = match num_live_neighbors {
                            0 ... 1 => Cell::Dead,
                            2 ... 3 => Cell::Live,
                            _ => Cell::Dead,
                        }
                    },
                    Cell::Dead => {
                        if num_live_neighbors == 3 {
                            new_field[y][x] = Cell::Live;
                        }
                    }
                }
            }
        }
        self.cells = new_field;
    }

    fn get_neighbors(
        &self,
        x: usize,
        y: usize,
    ) -> Vec<Cell> {
        vec!(
            self.get_north_neighbor(x, y),
            self.get_northeast_neighbor(x, y),
            self.get_east_neighbor(x, y),
            self.get_southeast_neighbor(x, y),
            self.get_south_neighbor(x, y),
            self.get_southwest_neighbor(x, y),
            self.get_west_neighbor(x, y),
            self.get_northwest_neighbor(x, y)
        )
    }

    fn get_north_neighbor(
        &self,
        x: usize,
        py: usize,
    ) -> Cell {
        let y = if py == 0 {
            self.height() - 1
        } else {
            py - 1
        };

        self.cells[y][x]
    }

    fn get_northeast_neighbor(
        &self,
        px: usize,
        py: usize,
    ) -> Cell {
        let y = if py == 0 {
            self.height() - 1
        } else {
            py - 1
        };
        let x = if px == self.width() - 1 {
            0
        } else {
            px + 1
        };

        self.cells[y][x]
    }

    fn get_east_neighbor(
        &self,
        px: usize,
        y: usize,
    ) -> Cell {
        let x = if px == self.width() - 1 {
            0
        } else {
            px + 1
        };

        self.cells[y][x]
    }

    fn get_southeast_neighbor(
        &self,
        px: usize,
        py: usize,
    ) -> Cell {
        let y = if py == self.height() - 1 {
            0
        } else {
            py + 1
        };
        let x = if px == self.width() - 1 {
            0
        } else {
            px + 1
        };

        self.cells[y][x]
    }

    fn get_south_neighbor(
        &self,
        x: usize,
        py: usize,
    ) -> Cell {
        let y = if py == self.height() - 1 {
            0
        } else {
            py + 1
        };

        self.cells[y][x]
    }

    fn get_southwest_neighbor(
        &self,
        px: usize,
        py: usize,
    ) -> Cell {
        let y = if py == self.height() - 1 {
            0
        } else {
            py + 1
        };
        let x = if px == 0 {
            self.width() - 1
        } else {
            px - 1
        };

        self.cells[y][x]
    }

    fn get_west_neighbor(
        &self,
        px: usize,
        y: usize,
    ) -> Cell {
        let x = if px == 0 {
            self.width() - 1
        } else {
            px - 1
        };

        self.cells[y][x]
    }

    fn get_northwest_neighbor(
        &self,
        px: usize,
        py: usize,
    ) -> Cell {
        let y = if py == 0 {
            self.height() - 1
        } else {
            py - 1
        };
        let x = if px == 0 {
            self.width() - 1
        } else {
            px - 1
        };

        self.cells[y][x]
    }

    pub fn awake(
        &mut self,
        x: usize,
        y: usize,
    ) {
        self.cells[y][x] = Cell::Live;
    }

    pub fn get(
        &self,
        x: usize,
        y: usize,
    ) -> Cell {
        self.cells[y][x]
    }

    pub fn width(&self) -> usize {
        self.cells[0].len()
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }
}
