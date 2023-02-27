#![allow(unused_doc_comments)]

pub mod main_ui {
    use tui::{
        backend::Backend,
        layout::{Alignment, Constraint, Corner, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Span, Spans},
        widgets::{Block, BorderType, Borders, Gauge, List, ListItem},
        Frame,
    };
    use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

    use crate::lib::{app::app_mod::StatefulList, config::config::DriveStruct};

    pub fn main_ui<B: Backend>(
        f: &mut Frame<B>,
        drives: &StatefulList<DriveStruct>,
        message: &str,
    ) {
        let size = f.size();

        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(size);

        let hchunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(vchunks[0]);

        let lchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(hchunks[0]);

        /**
         * ! Welcome block
         */
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightCyan))
                    .title(Span::styled(
                        "| # Welcome # |",
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::LightMagenta),
                    ))
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .label(Span::styled(
                "RRclone only supports mount 😁",
                Style::default(),
            ))
            .style(Style::default());
        // .gauge_style(Style::default().fg(Color::Yellow))
        // .use_unicode(true)
        f.render_widget(gauge, lchunks[0]);

        /**
         * ! Drives block
         */
        let items: Vec<ListItem> = drives
            .items
            .iter()
            .map(|i| {
                ListItem::new(Spans::from(i.name.clone())).style(Style::default().fg(Color::White))
            })
            .collect();

        let list_drives = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("List")
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .start_corner(Corner::TopLeft);
        // .highlight_symbol(">> ");
        f.render_stateful_widget(list_drives, lchunks[1], &mut drives.state.clone());

        /**
         * ! Bottom message
         */
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightRed))
                    .border_type(BorderType::Rounded),
            )
            .label(Span::styled(message, Style::default()))
            .style(Style::default());
        f.render_widget(gauge, vchunks[1]);

        /**
         * ! Logs block
         */
        let tui_w: TuiLoggerWidget = TuiLoggerWidget::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightCyan))
                    .title(Span::styled(
                        r"/ Logs \",
                        Style::default().fg(Color::LightRed),
                    ))
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .output_separator('~')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .output_file(false)
            .output_line(false)
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan));
            // .style(Style::default().fg(Color::White).bg(Color::Black))
        //     .state(&mut app.states[sel]);
        f.render_widget(tui_w, hchunks[1]);
    }
}
