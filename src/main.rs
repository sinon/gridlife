use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use game_of_life::{CellState, Grid};

#[derive(Debug, Default)]
pub struct App {
    grid: Grid<CellState>,
    run: bool,
    exit: bool,
    cycles: u32,
    population: u32,
    height: usize,
    width: usize,
}

impl App {
    pub fn new(height: usize, width: usize) -> Self {
        let mut grid = Grid::new_empty(width, height);
        let population = grid.update_states();
        App {
            grid,
            exit: false,
            run: false,
            cycles: 0,
            population,
            height,
            width,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            if self.run {
                self.cycle();
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => self.run_simulation(),
            KeyCode::Char('s') => self.stop_simulation(),
            KeyCode::Char('n') => self.cycle(),
            KeyCode::Char('?') => self.random_grid(),
            _ => {}
        }
    }
    fn cycle(&mut self) {
        self.population = self.grid.update_states();
        self.cycles += 1;
    }
    fn exit(&mut self) {
        self.exit = true;
    }
    fn run_simulation(&mut self) {
        self.run = true;
    }
    fn stop_simulation(&mut self) {
        self.run = false;
    }
    fn random_grid(&mut self) {
        self.grid = Grid::new_random(self.width, self.height);
        self.population = self.grid.update_states();
        self.cycles = 0;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Game of Life ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Run".into(),
            "<r>".blue().bold(),
            " Stop".into(),
            "<s>".blue().bold(),
            " Single Cycle".into(),
            "<n>".blue().bold(),
            " Regenerate".into(),
            "<?>".blue().bold(),
            " Population: ".into(),
            format!("{}", self.population).red().bold(),
            " Cycles: ".into(),
            format!("{} ", self.cycles).red().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let grid_out = self.grid.to_string();
        let lines: Vec<Line> = grid_out.lines().map(Line::from).collect();
        let grid_text = Text::from(lines);

        Paragraph::new(grid_text).block(block).render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let s = terminal.size()?;
    let app_result = App::new(s.height as usize - 1, s.width as usize - 1).run(&mut terminal);
    ratatui::restore();
    app_result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 100, 4));

        app.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Game of Life ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
        "┃💀💀💀💀💀💀💀💀💀💀                                                                              ┃",
        "┃💀💀💀💀💀💀💀💀💀💀                                                                              ┃",
        "┗━━━━━━━━━ Quit <Q>  Run<r> Stop<s> Single Cycle<n> Regenerate<?> Population: 0 Cycles: 0 ━━━━━━━━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().red().bold();
        let key_style = Style::new().blue().bold();
        // Game of Life
        expected.set_style(Rect::new(43, 0, 14, 1), title_style);
        // <Q>
        expected.set_style(Rect::new(16, 3, 4, 1), key_style);
        // <r>
        expected.set_style(Rect::new(24, 3, 3, 1), key_style);
        //<s>
        expected.set_style(Rect::new(32, 3, 3, 1), key_style);
        //<n>
        expected.set_style(Rect::new(48, 3, 3, 1), key_style);
        //<?>
        expected.set_style(Rect::new(62, 3, 3, 1), key_style);
        // 0
        expected.set_style(Rect::new(78, 3, 1, 1), counter_style);
        // 0
        expected.set_style(Rect::new(88, 3, 2, 1), counter_style);
        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('n').into());
        assert_eq!(app.cycles, 1);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
