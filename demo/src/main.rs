use crossterm::event::{Event, KeyCode};
use log::info;
use ratatui::{
    Terminal,
    backend::Backend,
    layout::{Constraint, Layout},
};
use tui_logger::{init_logger, set_default_level};

/// Custom components created and used by the Demo
mod components;

fn draw<T: Backend>(t: &mut Terminal<T>) {
    t.draw(|frame| {
        let layout = Layout::vertical([
            Constraint::Percentage(75),
            Constraint::Percentage(25),
        ]);
        let [top, bottom] = layout.areas(frame.area());

        frame.render_widget(components::root(), top);
        frame.render_widget(components::logger(), bottom);
    })
    .unwrap();
}

fn main() {
    init_logger(log::LevelFilter::Trace).unwrap();
    set_default_level(log::LevelFilter::Debug);

    info!("azptui Demo Started");

    ratatui::run(|terminal| {
        loop {
            draw(terminal);
            azptui::events::handle_event(crossterm::event::read().unwrap());
        }
    });
}
