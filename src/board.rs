use rand::Rng;

pub struct ConwayGame {
    pub width: usize,
    pub height: usize,
    pub game: Vec<u8>,
}

pub fn build_board(width: usize, height: usize) -> ConwayGame {
    ConwayGame {
        width,
        height,
        game: vec![0; width * height],
    }
}

impl ConwayGame {
    fn print(&self) {
        for line in 0..self.height {
            for column in 0..self.width {
                print!("{}", self.get_tile(line, column).unwrap());
            }
            println!("");
        }
    }

    pub fn generate_random(&mut self) {
        let mut rng = rand::thread_rng();

        for line in 0..self.height {
            for column in 0..self.width {
                self.set_tile(line, column, rng.gen_range(0..2))
            }
        }
    }

    fn get_tile(&self, line: usize, column: usize) -> Option<&u8> {
        let tile = self.game.get(line * self.width + column);

        match tile {
            None => {
                println!("Failed on line: {}, column: {}", line, column);
                None
            }
            Some(value) => Some(value),
        }
    }

    fn set_tile(&mut self, line: usize, column: usize, value: u8) {
        let tile = self.game.get_mut(line * self.width + column).unwrap();

        *tile = value;
    }

    fn get_neighbour_index(&self, line: usize, column: usize) -> Vec<usize> {
        let mut neighbours: Vec<usize> = Vec::new();
        let index = line * self.width + column;

        // Up
        if line > 0 {
            neighbours.push(index - self.width);
        }

        // Down
        if line < self.width {
            neighbours.push(index + self.width);
        }

        // Left
        if column > 0 {
            neighbours.push(index - 1);
        }

        // Right
        if column < self.width {
            neighbours.push(index + 1);
        }

        neighbours
    }

    fn neighbour_count(&self, line: usize, column: usize) -> u8 {
        let mut counter: u8 = 0;

        // Up
        if line > 0 {
            counter += self.get_tile(line - 1, column).unwrap();
        }

        // Down
        if line < self.height - 1 {
            counter += self.get_tile(line + 1, column).unwrap();
        }

        // Left
        if column > 0 {
            counter += self.get_tile(line, column - 1).unwrap();
        }

        // Right
        if column < self.width - 1 {
            counter += self.get_tile(line, column + 1).unwrap();
        }

        // Top Right
        if line > 0 && column < self.width - 1 {
            counter += self.get_tile(line - 1, column + 1).unwrap();
        }

        // Top Left
        if line > 0 && column > 0 {
            counter += self.get_tile(line - 1, column - 1).unwrap();
        }

        // Bottom Right
        if line < self.height - 1 && column < self.width - 1 {
            counter += self.get_tile(line + 1, column + 1).unwrap();
        }
        // Bottom Left
        if line < self.height - 1 && column > 0 {
            counter += self.get_tile(line + 1, column - 1).unwrap();
        }

        counter
    }

    fn print_neighbour_board(&mut self) {
        println!("------------------ Neighbour ------------------");
        for line in 0..self.height {
            for column in 0..self.width {
                print!("{}", self.neighbour_count(line, column));
            }
            println!("");
        }
        println!("-----------------------------------------------");
    }

    fn update_tile(&mut self, line: usize, column: usize) {
        let number_of_neigbours: u8 = self.neighbour_count(line, column);
        let tile = self
            .get_tile(line, column)
            .expect("Lookup failed in update_tile!");

        match (tile, number_of_neigbours) {
            (0, 3) => self.set_tile(line, column, 1u8),
            (1, 0 | 1) => self.set_tile(line, column, 0u8),
            (1, 2 | 3) => (),
            (1, _) => self.set_tile(line, column, 0u8),
            (_, _) => (),
        }
    }

    pub fn step(&mut self) {
        for line in 0..self.height {
            for column in 0..self.width {
                self.update_tile(line, column);
            }
        }
    }

    pub fn to_string(&mut self) -> String {
        let mut game_string = String::new();
        let mut current_tile;

        for line in 0..self.height {
            for column in 0..self.width {
                current_tile = self.get_tile(line, column).unwrap();
                match current_tile {
                    0 => game_string.push_str(" "),
                    _ => game_string.push_str("S"),
                }
            }
            game_string.push_str("\n");
        }

        game_string
    }

    pub fn to_string_neighbour(&mut self) -> String {
        let mut neighbour_string = String::new();

        for line in 0..self.height {
            for column in 0..self.width {
                neighbour_string.push_str(&self.neighbour_count(line, column).to_string());
            }
            neighbour_string.push_str("\n");
        }
        neighbour_string
    }
}
