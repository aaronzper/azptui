use log::info;
use ratatui::{
    Terminal,
    backend::Backend,
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::Block,
};
use tui_logger::{TuiLoggerWidget, init_logger, set_default_level};

/// Custom components created and used by the Demo
mod components;

fn draw<T: Backend>(t: &mut Terminal<T>) {
    tui_logger::move_events();
    t.draw(|frame| {
        let layout = Layout::vertical([
            Constraint::Percentage(75),
            Constraint::Percentage(25),
        ]);
        let [top, bottom] = layout.areas(frame.area());
        frame.render_widget(components::root(), top);
        frame.render_widget(
            TuiLoggerWidget::default().block(
                Block::bordered().title("Log").title_bottom(
                    Line::from(
                        "Everything outside this box rendered via AZPTUI! B\"SD",
                    )
                    .centered()
                    .italic(),
                ),
            ),
            bottom,
        );
    })
    .unwrap();
}

fn main() {
    init_logger(log::LevelFilter::Trace).unwrap();
    set_default_level(log::LevelFilter::Trace);

    info!("azptui Demo Started");

    ratatui::run(|terminal| {
        loop {
            draw(terminal);
            if crossterm::event::read().unwrap().is_key_press() {
                return;
            }
        }
    });
}
