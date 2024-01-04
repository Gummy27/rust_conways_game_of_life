mod board;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut board: board::ConwayGame = board::build_board(70);
    board.generate_random();

    let mut should_quit = false;
    let mut auto_step = false;
    while !should_quit {
        if auto_step {
            board.step();
        }

        terminal.draw(|f| {
            ui(&mut board, f);
        })?;
        should_quit = handle_events(&mut board, &mut auto_step)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(game: &mut board::ConwayGame, auto_step: &mut bool) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }

            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('n') {
                game.step();
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('a') {
                *auto_step = !*auto_step;
            }
        }
    }
    Ok(false)
}

fn ui(game: &mut board::ConwayGame, frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(frame.size());

    let active_game = Text::from(game.to_string());
    frame.render_widget(
        Paragraph::new(active_game)
            .block(Block::default().borders(Borders::ALL).title("Active Game"))
            .alignment(Alignment::Center),
        main_layout[0],
    );

    let neigbour_count = Text::from(game.to_string_neighbour());
    frame.render_widget(
        Paragraph::new(neigbour_count)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Neighbour Count"),
            )
            .alignment(Alignment::Center),
        main_layout[1],
    );
}
