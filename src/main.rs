use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    text::Text,
    layout::{Rect},
    widgets::{Block, Widget, Paragraph},
    symbols::border,
    DefaultTerminal, Frame,

};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::time::{Duration,SystemTime};
use std::io;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    Ok(app_result?)
}

#[derive(Debug, Default)]
pub struct App {
    timer: bool,
    exit: bool,
}

impl App {

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        };
        Ok(())
    }

    fn draw (&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('s') => self.start_timer(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        if self.timer == true {
            self.start_timer();
        }
        self.exit = true;
    }

    fn start_timer(&mut self) {
        if self.timer == true {
            todo!();
        } else {
            todo!();
        }
    }
}

impl Widget for &App {
    fn render(self, area:Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_set(border::THICK);
        let text = Text::from("q to quit");

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area,buf);
        //let time = Text::
        //todo!();

    }
}


