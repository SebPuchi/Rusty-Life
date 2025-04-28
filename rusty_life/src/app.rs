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

use std::thread;
use std::time::Duration;

use crate::grid::LifeGrid;


pub struct App {
    frame_area: Rect,
    inner_area: Rect,
    grid: LifeGrid,
    paused: bool,
    exit: bool,
    pub status: io::Result<()>
}

//App behaviour
impl App {
    pub fn new(terminal: &mut DefaultTerminal) -> Self {
        let current_frame_area = terminal
            .get_frame()
            .area();

        let outer_block = Block::default()
            .borders(Borders::ALL)
            .title_bottom(TextLine::from(""))
            .title_bottom(TextLine::from(""));

        let current_inner_area = outer_block.inner(current_frame_area);

        Self {
            frame_area: current_frame_area,
            inner_area: current_inner_area,
            paused: true,
            exit: false,
            grid: LifeGrid::new(current_inner_area.height, current_inner_area.width),
            status: Ok(()),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        //self.grid.spawn_r_center();
        //self.grid.spawn_glider_center();
        self.grid.spawn_bc_center();

        while !self.exit {
            // fetch and draw next generation 
            if !self.paused {
                self.grid.evolve_next();
            }

            let live_coords = self.grid.build_coords();
            terminal.draw(|frame| self.ui(frame, &live_coords))?;
            self.handle_events()?;
            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(5))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(' ') => self.toggle_pause(),
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {}
        }
    }

    fn toggle_pause(&mut self){
        self.paused = !self.paused;
    }

    fn exit(&mut self) {
        self.exit = true;
    }

}

//App rendering
impl App {
    fn ui(&self, frame: &mut Frame, alive_coords: &Vec<(f64,f64)>) {

        let outer_block = Block::default()
            .borders(Borders::ALL)
            .title_bottom(
                TextLine::from(format!(" generation: {} ", self.grid.generation))
                    .right_aligned(),
            )
            .title_bottom(
                TextLine::from(format!(
                    " frame_width: {} | frame_height: {} ",
                    self.grid.width, self.grid.height
                ))
                .left_aligned(),
            );
        
        frame.render_widget(&outer_block, self.frame_area);
        
        let inner_block = Canvas::default()
            .marker(symbols::Marker::HalfBlock)
            .paint(|ctx| {
                 ctx.draw(&Points {
                    coords: alive_coords,
                    color: Color::Red,
                });
            })
            .x_bounds([0.0, (self.inner_area.width  as f64) - 1.0])
            .y_bounds([0.0, (self.inner_area.height as f64 * 2.0) - 1.0]);

        frame.render_widget(inner_block, self.inner_area);
    }
}


