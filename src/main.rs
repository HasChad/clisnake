use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    widgets::{
        canvas::{Canvas, Circle, Points, Rectangle},
        Block, BorderTy,
    },
    DefaultTerminal,
};
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App<'a> {
    playground: Rect,
    head: Points<'a>,
    body: Points<'a>,
    food: Points<'a>,
    tick_count: u64,
}

impl<'a> App<'a> {
    fn new() -> Self {
        Self {
            playground: Rect::new(0, 0, 200, 100),
            head: Points {
                coords: &[(100.0, 50.0)],
                color: Color::White,
            },
            body: Points {
                coords: &[],
                color: Color::Gray,
            },
            food: Points {
                coords: &[(10.0, 10.0)],
                color: Color::Red,
            },
            tick_count: 0,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        break Ok(());
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('w') => self.head.coords. += 1.,
            KeyCode::Char('s') => self.head.y -= 1.,
            KeyCode::Char('d') => self.head.x += 1.,
            KeyCode::Char('a') => self.head.x -= 1.,
            _ => {}
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self.pong_canvas(), frame.area());
    }

    fn pong_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(
                Block::bordered()
                    .title("Clisnake")
                    .title_style(Style::new().yellow().bold())
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .paint(|ctx| {
                ctx.draw(&self.head);
                ctx.draw(&self.left_player);
                ctx.draw(&self.right_player);
            })
            .x_bounds([0., f64::from(self.playground.width)])
            .y_bounds([0., f64::from(self.playground.height)])
    }
}
