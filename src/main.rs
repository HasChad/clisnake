use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    widgets::{
        canvas::{Canvas, Circle},
        Block, BorderType,
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

struct App {
    ball: Circle,
    playground: Rect,
    vx: f64,
    vy: f64,
    tick_count: u64,
}

impl App {
    fn new() -> Self {
        Self {
            ball: Circle {
                x: 50.0,
                y: 40.0,
                radius: 10.0,
                color: Color::Yellow,
            },
            playground: Rect::new(10, 10, 200, 100),
            vx: 1.0,
            vy: 1.0,
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

    fn on_tick(&mut self) {
        self.tick_count += 1;

        // bounce the ball by flipping the velocity vector
        let ball = &self.ball;
        let playground = self.playground;

        if ball.x - ball.radius < f64::from(playground.left())
            || ball.x + ball.radius > f64::from(playground.right())
        {
            self.vx = -self.vx;
        }
        if ball.y - ball.radius < f64::from(playground.top())
            || ball.y + ball.radius > f64::from(playground.bottom())
        {
            self.vy = -self.vy;
        }

        self.ball.x += self.vx;
        self.ball.y += self.vy;

        // TODO: make the players move
    }

    fn draw(&self, frame: &mut Frame) {
        let pong_area = frame.area();

        frame.render_widget(self.pong_canvas(), pong_area);
    }

    fn pong_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(
                Block::bordered()
                    .title("Pong")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Double),
            )
            .paint(|ctx| {
                ctx.draw(&self.ball);
                ctx.print(10.0, 40.0, format!("ball pos = {}", self.ball.x));
            })
            .x_bounds([
                self.playground.x.into(),
                f64::from(self.playground.width) + 10.,
            ])
            .y_bounds([
                self.playground.y.into(),
                f64::from(self.playground.height) + 10.,
            ])
    }
}
