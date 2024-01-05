mod board;

fn setup_menu<B: Backend>(
    game: &mut board::ConwayGame,
    terminal: &mut Terminal<B>,
) -> io::Result<()> {
    let mut should_continue = false;

    while !should_continue {
        terminal.draw(|f| {
            setup_ui(&mut board, f);
        });
        handle_events_ui(&mut board, &mut should_continue);
    }

    Ok(())
}

fn handle_events_ui(game: &mut board::ConwayGame, should_continue: &mut bool) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Left => (),
                    KeyCode::Right => (),
                    KeyCode::Up => (),
                    KeyCode::Down => (),
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
