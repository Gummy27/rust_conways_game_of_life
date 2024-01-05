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

    let mut board: board::ConwayGame = board::build_board(10, 10);
    board.generate_random();

    let mut should_quit = false;
    let mut auto_step = false;
    while !should_quit {
        if auto_step {
            board.step();
        }

        terminal.draw(|f| {
            game_ui(&mut board, f);
        })?;
        should_quit = handle_events(&mut board, &mut auto_step)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn setup_menu<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut should_continue = false;

    while !should_continue {
        terminal.draw(setup_ui);
    }

    Ok(())
}

fn handle_events(game: &mut board::ConwayGame, auto_step: &mut bool) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('a') => *auto_step = !*auto_step,
                    KeyCode::Char('n') => game.step(),
                    _ => (),
                }
            }
        }
    }
    Ok(false)
}

fn game_ui(game: &mut board::ConwayGame, frame: &mut Frame) {
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

fn setup_ui(frame: &mut Frame) {
    let setup_layout =
        Layout::new(Direction::Horizontal, [Constraint::Percentage(90)]).split(frame.size());

    frame.render_widget(
        Paragraph::new(Text::from(
            "The arrow keys to shrink or expand the game board",
        ))
        .block(Block::default().borders(Borders::ALL).title("Setup"))
        .alignment(Alignment::Center),
        setup_layout[0],
    );
}
