pub mod main_ui {
    use tui::{
        backend::Backend,
        layout::{Alignment, Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::Span,
        widgets::{Block, BorderType, Borders},
        Frame,
    };

    pub fn main_ui<B: Backend>(f: &mut Frame<B>) {
        let size = f.size();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        f.render_widget(block, size);

        let vchunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
            .split(size);

        let lchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
            .split(vchunks[0]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightCyan))
            .title(Span::styled(
                "| Welcome |",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            ))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, lchunks[0]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightCyan))
            .title(Span::styled(
                " List of Drives ",
                Style::default().fg(Color::Cyan),
            ))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, lchunks[1]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightCyan))
            .title(Span::styled(
                r"/ Logs \",
                Style::default().fg(Color::LightRed),
            ))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, vchunks[1]);
    }
}
