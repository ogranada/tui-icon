#[cfg(test)]
mod tui_icon {
    use std::error::Error;
    use tui::{
        backend::TestBackend,
        layout::{Constraint, Direction, Layout},
        widgets::{Block, Borders, BorderType, canvas::Canvas},
        Terminal, buffer::Buffer,
    };
    use tui_icon::tui_icon::Icon;

    // To catch panic use #[should_panic]

    #[test]
    fn creation() -> Result<(), Box<dyn Error>> {
        let fp = "./assets/target/clock.pbm";
        let icon = Icon::new(fp, None, Some((0 as usize, 0 as usize)));

        let backend = TestBackend::new(10, 6);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1)].as_ref())
                .split(f.size());
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let canvas = Canvas::default()
                .block(block)
                .paint(|ctx| {
                    ctx.draw(&icon);
                })
                .x_bounds([0.0, icon.width as f64])
                .y_bounds([0.0, icon.height as f64]);
            f.render_widget(canvas, chunks[0]);
        })?;

        let expected = Buffer::with_lines(vec![
            "╭────────╮",
            "│ ⠐⣄⠤⠤⣠⠂ │",
            "│ ⡜  ⡄ ⢣ │",
            "│ ⢣ ⠈⠋⠉⡜ │",
            "│ ⠒⠉⠒⠒⠉⠒ │",
            "╰────────╯",
        ]);
        terminal.backend().assert_buffer(&expected);
    
        Ok(())
    }

}
