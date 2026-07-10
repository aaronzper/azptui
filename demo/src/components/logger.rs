use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Widget},
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

#[azptui::component]
pub fn logger() -> impl Widget {
    // TODO make this an event listener somehow so we re-render when render memo
    // gets set up
    tui_logger::move_events();

    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_info(Style::default().fg(Color::White))
        .style_debug(Style::default().fg(Color::LightBlue))
        .style_trace(Style::default().fg(Color::Magenta))
        .output_level(Some(TuiLoggerLevelOutput::Long))
        .output_timestamp(Some("%H:%M:%S".to_string()))
        .output_file(false)
        .output_line(false)
        .output_separator(' ')
        .block(
            Block::bordered()
                .title("Log")
                .title_bottom(Line::from("B\"SD").centered().italic()),
        )
}
