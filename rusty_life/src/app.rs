use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Circle, Map, MapResolution, Points, Rectangle},
        Block, 
        Borders, 
        BorderType,
    },
    layout::Rect,
    text::Line as TextLine,
    symbols,
    DefaultTerminal, 
    Frame
};

use crate::grid::LifeGrid;


pub struct App {
    frame_area: Rect,
    grid: LifeGrid,
    exit: bool,
    pub status: io::Result<()>
}

//App behaviour
impl App {
    pub fn new(terminal: &mut DefaultTerminal) -> Self {
        let current_frame_area = terminal
            .get_frame()
            .area();
        Self {
            frame_area: current_frame_area,
            exit: false,
            grid: LifeGrid::new(current_frame_area.height, current_frame_area.width),
            status: Ok(()),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            // fetch and draw next generation 
            terminal.draw(|frame| self.ui(frame))?;
            self.grid.generation+=1;
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

        let map = Canvas::default()
            .block(Block::bordered()
                .title_bottom(TextLine::from(
                    format!(
                        " generation: {} ", 
                        self.grid.generation
                    )
                ).right_aligned())
                .title_bottom(TextLine::from(
                    format!(
                        " frame_width: {} | frame_height: {} ", 
                        self.grid.width,
                        self.grid.height
                        
                    )
                ).left_aligned())
            )
            .marker(symbols::Marker::HalfBlock)
            .paint(|ctx| {
                 ctx.draw(&Points {
                    coords: &[(0.0, 0.0),(self.grid.width.into(), 0.0), (0.0, self.grid.height.into()), (self.grid.width.into(), self.grid.height.into())],
                    color: Color::Red,
                });
            }).x_bounds([0.0, (self.grid.width as f64)])  
            .y_bounds([0.0, (self.grid.height as f64)]);
            frame.render_widget(map, self.frame_area);
    }
}


