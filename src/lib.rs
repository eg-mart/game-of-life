pub struct Board {
    height: usize,
    width: usize,
    content: Vec<Vec<bool>>,
}

 pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, board: Option<Vec<Vec<bool>>>) -> Board {
        let mut content = vec![];

        match board {
            Some(x) => content = x,
            None => {
                let row = vec![false; width];
                content.resize(height, row);
            }
        }

        Board {
            width,
            height,
            content,
        }
    }

    pub fn calculate_next_state(&mut self) {
        let old_content = self.content.clone();
        for y in 0..self.height{
            for x in 0..self.width {
                let mut neighbors_cnt: u8 = 0;
                for offset_y in -1..=1 {
                    for offset_x in -1..=1 {
                        if offset_x == 0 && offset_y == 0 {
                            continue;
                        }

                        let neighbor_x = (x as i32 + offset_x).rem_euclid(self.width as i32) as usize;
                        let neighbor_y = (y as i32 + offset_y).rem_euclid(self.height as i32) as usize;

                        if old_content[neighbor_y][neighbor_x] {
                            neighbors_cnt += 1;
                        }
                    }
                }

                if neighbors_cnt == 3 {
                    self.content[y][x] = true;
                }
                else if neighbors_cnt >= 4 || neighbors_cnt <= 1 {
                    self.content[y][x] = false;
                }
            }
        }
    }

    pub fn toggle_cell(&mut self, cell: Cell) -> Result<(), String> {
        if !cell.x < self.width || !cell.y < self.height {
            return Err("Cell is out of board's bounds".to_string());
        }
        self.content[cell.y][cell.x] = !self.content[cell.y][cell.x];
        Ok(())
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn content(&self) -> &Vec<Vec<bool>> {
        &self.content
    }

    pub fn set_width(&mut self, width: usize) -> Result<(), String> {
        if width == 0 {
            return Err("Board width can't be 0".to_string());
        }

        self.width = width;
        for row in &mut self.content {
            row.resize(width, false);
        }

        Ok(())
    }

    pub fn set_height(&mut self, height: usize) -> Result<(), String> {
        if height == 0 {
            return Err("Board height can't be 0".to_string());
        }

        self.height = height;
        let empty_row = vec![false; self.width];
        self.content.resize(height, empty_row);

        Ok(())
    }
}
