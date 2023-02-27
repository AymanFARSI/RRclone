#![allow(unused_doc_comments)]

pub mod drive_ui {
    use chrono::SecondsFormat;
    use tui::{
        backend::Backend,
        layout::{Alignment, Constraint, Corner, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Span, Spans},
        widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph},
        Frame,
    };

    use crate::lib::{app::app_mod::StatefulList, config::config::DriveStruct};

    pub fn drive_ui<B: Backend>(
        f: &mut Frame<B>,
        drives: &StatefulList<DriveStruct>,
        message: &str,
        name: String,
        inser_mode: bool,
    ) {
        let size = f.size();

        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
            .split(size);

        let hchunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(vchunks[1]);

        let hvchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(hchunks[0]);

        /**
         * ! Top message
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
        f.render_widget(gauge, vchunks[0]);

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
                    .title(" Drives list ")
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            )
            .highlight_symbol(">> ")
            .start_corner(Corner::TopLeft);
        // .highlight_symbol(">> ");
        f.render_stateful_widget(list_drives, hvchunks[0], &mut drives.state.clone());

        let input = Paragraph::new(name.to_owned())
            .block(
                Block::default()
                    .title(" Add drive -> Enter ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(match inser_mode {
                        true => Style::default().fg(Color::Green),
                        false => Style::default(),
                    }),
            )
            .style(Style::default());
        f.render_widget(input, hvchunks[1]);

        let rchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .split(hchunks[1]);
        let rvchunks: Vec<Vec<Rect>> = rchunks
            .iter()
            .map(|f| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                    .split(*f)
            })
            .collect();

        let gauge = Gauge::default()
            .label(Span::styled("Name:", Style::default()))
            .style(Style::default());
        f.render_widget(gauge, rvchunks[0][0]);

        let gauge = Gauge::default()
            .label(Span::styled("Type:", Style::default()))
            .style(Style::default());
        f.render_widget(gauge, rvchunks[1][0]);

        let gauge = Gauge::default()
            .label(Span::styled("Expiry:", Style::default()))
            .style(Style::default());
        f.render_widget(gauge, rvchunks[2][0]);

        let i = drives.state.selected().unwrap();

        let gauge = Gauge::default()
            .label(Span::styled(drives.items[i].name.clone(), Style::default()))
            .style(Style::default());
        f.render_widget(gauge, rvchunks[0][1]);

        let gauge = Gauge::default()
            .label(Span::styled(
                drives.items[i].drive_type.clone(),
                Style::default(),
            ))
            .style(Style::default());
        f.render_widget(gauge, rvchunks[1][1]);

        let mut expiry = drives.items[i]
            .token
            .expiry
            .clone()
            .to_rfc3339_opts(SecondsFormat::AutoSi, false);
        expiry = expiry[0..16].replace("T", " ").replace("-", "/");
        let gauge = Gauge::default()
            .label(Span::styled(&expiry, Style::default()))
            .style(Style::default());
        f.render_widget(gauge, rvchunks[2][1]);
    }

    // fn inactivate(textarea: &mut TextArea<'_>) {
    //     textarea.set_cursor_line_style(Style::default());
    //     textarea.set_cursor_style(Style::default());
    //     let b = textarea
    //         .block()
    //         .cloned()
    //         .unwrap_or_else(|| Block::default().borders(Borders::ALL));
    //     textarea.set_block(
    //         b.style(Style::default().fg(Color::DarkGray))
    //             .title(" Inactive (^X to switch) "),
    //     );
    // }

    // fn activate(textarea: &mut TextArea<'_>) {
    //     textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    //     textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    //     let b = textarea
    //         .block()
    //         .cloned()
    //         .unwrap_or_else(|| Block::default().borders(Borders::ALL));
    //     textarea.set_block(b.style(Style::default()).title(" Active "));
    // }
}
