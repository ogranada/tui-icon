use rand::Rng;
use std::{collections::HashMap, error::Error, fs, io};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{canvas::Canvas, Block, BorderType, Borders},
    Terminal,
};
use tui_icon::tui_icon::{Icon, IconColor};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let paths = fs::read_dir("./assets/target").unwrap();

    let offset_elms: Vec<_> = vec![
        ("./assets/code-branch-solid_16x16.pbm", (2, 0)),
        ("./assets/cut-solid_16x16.pbm", (1, 0)),
        ("./assets/file-solid_16x16.pbm", (2, 0)),
        ("./assets/trash-alt-solid_16x16.pbm", (1, 0)),
        ("./assets/trash-restore-alt-solid_16x16.pbm", (1, 0)),
        ("./assets/trash-restore-solid_16x16.pbm", (1, 0)),
        ("./assets/folder-open-solid_16x16.pbm", (0, 2)),
    ];

    let offset_hmap: HashMap<&str, (i32, i32)> = HashMap::from_iter(offset_elms);

    let icons = paths
        .map(|path| {
            let partial = path.unwrap().path();
            let elm = partial.display();
            let elm = format!("{}", elm);
            if elm.ends_with("pbm") {
                let fp: &str = elm.as_str();
                let r = rng.gen_range(70..255);
                let g = rng.gen_range(70..255);
                let b = rng.gen_range(70..255);
                let color = Some(IconColor::new(r, g, b));
                let offset = offset_hmap.get(fp).unwrap_or(&(0, 0));
                Some(Icon::new(
                    fp,
                    color,
                    Some((offset.0 as usize, offset.1 as usize)),
                ))
            } else {
                None
            }
        })
        .filter(|elm| elm.is_some());

    let icons: Vec<Icon> = icons.map(|elm| elm.unwrap()).collect();

    let blk_size = 16;
    let vsize = ((blk_size / 8) * 3) as u16;
    let hsize = ((blk_size / 8) * 5) as u16;

    let rows_count = ((((icons.len() as f32) / 6.0) as f32).ceil() as i32) * 2;
    let row_iter_count = rows_count as usize / 2;

    let rows_constraints: Vec<Constraint> = (-1..rows_count)
        .map(|val| Constraint::Length(if val % 2 == 0 { vsize } else { 1 }))
        .collect();

    enable_raw_mode()?;
    let stdout = io::stdout();
    let mut backend = CrosstermBackend::new(stdout);
    backend.clear()?;
    let mut terminal = Terminal::new(backend)?;

    //*
    terminal.draw(|f| {
        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                rows_constraints.as_ref(),
            )
            .split(f.size());
        for r in 0..row_iter_count {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(hsize),
                        Constraint::Length(1),
                        Constraint::Length(hsize),
                        Constraint::Length(1),
                        Constraint::Length(hsize),
                        Constraint::Length(1),
                        Constraint::Length(hsize),
                        Constraint::Length(1),
                        Constraint::Length(hsize),
                        Constraint::Length(1),
                        Constraint::Length(hsize),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(vchunks[2 * r + 1]);

            for c in 0..6 {
                let p_icon = icons.get(r * 6 + c);
                if let Some(icon) = p_icon {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded);
                    let canvas = Canvas::default()
                        .block(block)
                        .paint(|ctx| {
                            ctx.draw(icon);
                        })
                        .x_bounds([0.0, icon.width as f64])
                        .y_bounds([0.0, icon.height as f64]);
                    f.render_widget(canvas, chunks[2 * c + 1]);
                }
            }
        }
    })?;
    // */
    disable_raw_mode()?;
    terminal.show_cursor()?;
    println!("\n\n");

    Ok(())
}
