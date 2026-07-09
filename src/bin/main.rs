use azptui::hello;
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    layout::{Constraint, Layout, Rect},
};

fn draw<T: Backend>(t: &mut Terminal<T>) {
    t.draw(|frame| {
        let layout = Layout::vertical([
            Constraint::Percentage(75),
            Constraint::Percentage(25),
        ]);
        let [top, bottom] = layout.areas(frame.area());
        frame.render_widget(hello(), top);
        frame.render_widget("Will be azptui monitoring, bez\"h", bottom);
    })
    .unwrap();
}

fn main() {
    ratatui::run(|terminal| {
        loop {
            draw(terminal);
            if crossterm::event::read().unwrap().is_key_press() {
                return;
            }
        }
    });
}
