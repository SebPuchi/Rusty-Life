use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Circle, Map, MapResolution, Points, Rectangle},
        Block, 
        Borders, 
        BorderType
    }, 
    layout::{Rect},
    buffer::Buffer,
    DefaultTerminal, 
    Frame
};

use crate::grid::LifeGrid;


pub struct App {
    grid: LifeGrid,
    exit: bool,
    cell_size: u16
}

//App behaviour
impl App {
    pub fn new(grid: LifeGrid) -> Self {
        Self {
            exit: false,
            grid,
            cell_size: 10,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

}

//App rendering
impl App {
    fn ui(&self, frame: &mut Frame) {
        let size = frame.area(); // Get terminal size in character cells
        let scaled_width = size.width / 2;

        let bounds = Rect::new(0, 0, scaled_width, scaled_width / 2);

        let map = Canvas::default()
            .block(Block::bordered().title("World"))
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::Green,
                    resolution: MapResolution::High,
                });
                ctx.print(100.0, -100.0, "You are here");
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0]);
            frame.render_widget(map, bounds);
    }
}


