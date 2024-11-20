use std::fs::read_to_string;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget},
    DefaultTerminal,
};

fn main() {
    let terminal = ratatui::init();
    let logo = read_to_string("logo.txt").expect("logo.txt is missing");
    App::new(&logo).run(terminal);
    ratatui::restore();
}

struct App<'a> {
    should_exit: bool,
    logo: &'a str,
    login_list: LoginList,
}

struct LoginList {
    items: [String; 2],
    state: ListState,
}

impl<'a> App<'a> {
    fn new(logo: &'a str) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            should_exit: false,
            logo,
            login_list: LoginList {
                items: ["Login".to_string(), "Register".to_string()],
                state: list_state,
            },
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) {
        while !self.should_exit {
            terminal
                .draw(|frame| frame.render_widget(&mut self, frame.area()))
                .unwrap();
            if let Event::Key(key) = event::read().unwrap() {
                self.handle_key(key);
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.login_list.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.login_list.state.select_previous(),
            _ => {}
        }
    }
}

impl<'a> Widget for &mut App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [_, logo_row, _, list_row, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(11),
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Fill(1),
        ])
        .areas(area);
        let [_, list_column, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ])
        .areas(list_row);

        let [_, logo_column, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(78),
            Constraint::Fill(1),
        ])
        .areas(logo_row);

        let items: Vec<ListItem> = self
            .login_list
            .items
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect();

        let list = List::new(items)
            .block(Block::bordered())
            .style(Style::new().white())
            .highlight_style(Style::new().blue().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, list_column, buf, &mut self.login_list.state);

        let logo = Text::from(self.logo).blue();
        Widget::render(logo, logo_column, buf);
    }
}
