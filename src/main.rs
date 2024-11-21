use std::{fs::read_to_string, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::Widget,
    DefaultTerminal,
};

use crate::views::{landing::LandingView, register::RegisterView};

pub mod views;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let logo = read_to_string("logo.txt").expect("logo.txt is missing");
    let app_result = App::new(logo).run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Clone, Copy)]
pub enum AppState {
    Landing,
    Register,
}

struct App {
    should_exit: bool,
    state: AppState,

    landing_view: LandingView,
    register_view: RegisterView,
}

impl App {
    fn new(logo: String) -> Self {
        Self {
            should_exit: false,
            state: AppState::Landing,

            landing_view: LandingView::new(logo),
            register_view: RegisterView::default(),
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if let KeyCode::Char('q') = key.code {
            self.should_exit = true
        }

        match &mut self.state {
            AppState::Landing => {
                self.landing_view.handle_key(key);
                if let Some(state) = self.landing_view.should_go_to {
                    self.state = state;
                }
                self.landing_view.should_go_to = None;
            }
            AppState::Register => {
                self.register_view.handle_key(key);
                if let Some(state) = self.register_view.should_go_to {
                    self.state = state;
                }
                self.register_view.should_go_to = None;
            }
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match &mut self.state {
            AppState::Landing => self.landing_view.render(area, buf),
            AppState::Register => {}
        }
    }
}
