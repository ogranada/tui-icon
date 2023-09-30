
# TUI Icon

This is a library that convert pbm (portable bit map) image and print it into the console.

```rust

fn main() {
    // prepare icon
    let fp = "./assets/target/clock.pbm";
    let icon = Icon::new(fp, None, Some((0 as usize, 0 as usize)));
    // do TUI lib stuff
    // ...
    // ...
    // ...

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
                ctx.draw(&icon); // Here the icon is drawn.
            })
            .x_bounds([0.0, icon.width as f64])
            .y_bounds([0.0, icon.height as f64]);
        f.render_widget(canvas, chunks[0]);
    })?;
}

```