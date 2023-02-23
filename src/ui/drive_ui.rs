pub mod drive_ui {
    use tui::{
        backend::Backend,
        layout::Alignment,
        widgets::{Block, BorderType, Borders},
        Frame,
    };

    pub fn drive_ui<B: Backend>(f: &mut Frame<B>) {
        // Wrapping block for a group
        // Just draw the block and the group on the same area and build the group
        // with at least a margin of 1
        let size = f.size();

        // Surrounding block
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Manage Drive")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, size);
    }
}
