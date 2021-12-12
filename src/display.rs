use termion::{input::MouseTerminal, raw::RawTerminal, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};

use crate::{app::Item, net::Resolver, App};

fn gen_rows<'a>(items: &Vec<Item>, resolver: &Resolver) -> Vec<Row<'a>> {
    let mut rows = Vec::new();
    for i in items {
        let entry = vec![
            format!("{}:{}", i.ip.to_string(), i.port.to_string()),
            resolver.resolve_ip(i.ip),
            gen_bytes_str(i.tot_rx),
            gen_bytes_str(i.tot_tx),
        ];
        let cells = entry.iter().map(|c| Cell::from(c.clone()));
        rows.push(Row::new(cells).height(1));
    }

    rows
}

pub fn draw_terminal(
    terminal: &mut Terminal<
        TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<std::io::Stdout>>>>,
    >,
    app: &mut App,
) -> anyhow::Result<()> {
    terminal.draw(|f| {
        let rects = Layout::default()
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .margin(1)
            .split(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().add_modifier(Modifier::BOLD);
        let header_cells = ["Peer", "Name", "Total received", "Total sent"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = gen_rows(&app.items.vec, &app.resolver);
        let peer_count = rows.len();

        let header_text = Span::styled(
            format!("Process: {} [{} peers]", app.process_name, peer_count),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

        let header_paragraph = Paragraph::new(header_text).alignment(Alignment::Left);

        let t = Table::new(rows.into_iter())
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Peer utilization"),
            )
            .highlight_style(selected_style)
            // .highlight_symbol("* ")
            .widths(&[
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]);

        f.render_widget(header_paragraph, rects[0]);
        f.render_stateful_widget(t, rects[1], &mut app.state);
    })?;

    Ok(())
}
// }

pub fn gen_bytes_str(kb: u64) -> String {
    let mut bytes_str = format!("{} kiB", kb);
    if kb >= 1024 {
        bytes_str = format!("{:.2} MiB", kb as f32 / 1024f32);
    }

    if kb >= 1024 * 1024 {
        bytes_str = format!("{:.2} GiB", kb as f32 / 1024f32 / 1024f32);
    }

    bytes_str
}
