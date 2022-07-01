use sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time;
use std::process;
use sdl2::render;
use sdl2::video;
use sdl2::mouse;
use game_of_life::Board;
use game_of_life::Cell;

pub enum Mode {
    Normal,
    Edit,
}

pub struct BoardSprite {
    board: Board,
    mode: Mode,
    cell_size: usize,
}

impl BoardSprite {
    pub fn new(board: Board, mode: Mode, cell_size: usize) -> BoardSprite {
        BoardSprite {
            board,
            mode,
            cell_size,
        }
    }

    pub fn draw_board<T: render::RenderTarget>(&self, canvas: &mut render::Canvas<T>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
                
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if self.board.content()[y][x]{
                    canvas.fill_rect(Rect::new(
                        (x * self.cell_size) as i32,
                        (y * self.cell_size) as i32, 
                        self.cell_size as u32, 
                        self.cell_size as u32
                    ))?;
                }
            }
        }

        Ok(())
    }

    pub fn draw_grid<T>(&self, canvas: &mut render::Canvas<T>) -> Result<(), String>
        where T: render::RenderTarget
    {
        canvas.set_draw_color(Color::RGB(50, 50, 50));

        for x in 0..self.board.width() + 1{
            canvas.draw_line(
                Point::new((x * self.cell_size) as i32, 0),
                Point::new((x * self.cell_size) as i32, (self.board.height() * self.cell_size) as i32),
            )?;
        }

        for y in 1..self.board.height() + 1{
            canvas.draw_line(
                Point::new(0, (y * self.cell_size) as i32),
                Point::new((self.board.width() * self.cell_size) as i32, (y * self.cell_size) as i32),
            )?;
        }

        Ok(())
    }

    pub fn highlight_cell<T>(&self, cell: Cell, canvas: &mut render::Canvas<T>) -> Result<(), String>
        where T: render::RenderTarget
    {
        if cell.x >= self.board.width() || cell.y >= self.board.height() {
            return Err("Cell is out of board's bounds".to_string());
        }

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.draw_rect(Rect::new(
            (cell.x * self.cell_size) as i32,
            (cell.y * self.cell_size) as i32,
            self.cell_size as u32,
            self.cell_size as u32
        ))?;

        Ok(())
    }

    pub fn point_to_cell(&self, point: Point) -> Result<Cell, String> {
        let x: usize = (point.x() / self.cell_size as i32) as usize;
        let y: usize = (point.y() / self.cell_size as i32) as usize;

        if x >= self.board.width() || y >= self.board.height() {
            return Err("Point is out of board's bounds".to_string());
        }

        Ok(Cell { x, y })
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("Failed to create an OpenGL context");
    let video_system = sdl_context.video().expect("Failed to initialize video subsystem");
    let mut canvas = video_system.window("Game of life", 600u32, 600u32)
        .build().expect("Failed to open a window") 
        .into_canvas().build().expect("Failed to acces window's canvas");
    let mut event_pump = sdl_context.event_pump().expect("Failed to initialize event subsystem");
    
    let board = Board::new(40, 40, None);
    let mut board_sprite = BoardSprite::new(board, Mode::Normal, 15);
    let mut frame_count = 0;

    loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        board_sprite.draw_board::<video::Window>(&mut canvas).expect("Failed to draw window's contents"); 
        board_sprite.draw_grid(&mut canvas).expect("Failed to draw window's contents");

        if let Mode::Edit = board_sprite.mode {
            let mouse_state = &event_pump.mouse_state();
            let mouse_cell = board_sprite.point_to_cell(Point::new(mouse_state.x(), mouse_state.y()));
            match mouse_cell {
                Ok(cell) => board_sprite.highlight_cell(cell, &mut canvas).expect("Failed to draw window's contents"),
                Err(_) => {}
            }
        }

        for event in event_pump.poll_iter(){
            match event {
                Event::Quit {..} => process::exit(0),
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    match board_sprite.mode {
                        Mode::Edit => board_sprite.mode = Mode::Normal,
                        Mode::Normal => board_sprite.mode = Mode::Edit,
                    }
                }
                Event::MouseButtonDown { mouse_btn: mouse::MouseButton::Left, x, y, .. } => {
                    if let Mode::Edit = board_sprite.mode {
                        let mouse_cell = board_sprite.point_to_cell(Point::new(x, y));
                        match mouse_cell {
                            Ok(cell) => board_sprite.board.toggle_cell(cell).unwrap(),
                            Err(_) => {},
                        }
                    }
                }
                _ => {}
            }
        }
        
        if let Mode::Normal = board_sprite.mode {
            if frame_count % 8 == 0 {
                board_sprite.board.calculate_next_state();
                frame_count = 1;
            }
        }

        frame_count += 1;
        canvas.present();
        thread::sleep(time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
