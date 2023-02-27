pub mod error_ui {
    use tui::{
        backend::Backend,
        style::{Color, Style},
        text::Span,
        widgets::{Block, BorderType, Borders, Gauge},
        Frame,
    };

    pub fn error_ui<B: Backend>(f: &mut Frame<B>, width: u16, height: u16) {
        let size = f.size();

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightRed))
                    .border_type(BorderType::Rounded),
            )
            .label(Span::styled(
                format!("{}x{} - please resize 😉 to at least 80x21", width, height),
                Style::default().fg(Color::Red),
            ))
            .style(Style::default());
        f.render_widget(gauge, size);
    }
}
