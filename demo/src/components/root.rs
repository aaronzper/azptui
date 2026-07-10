use azptui::{on_event, use_counter};
use crossterm::event::Event;
use log::info;
use ratatui::widgets::List;

#[azptui::component]
pub fn root() -> List<'static> {
    let counter = use_counter!();
    let old = sub();

    let list = if counter < 10 {
        ["< 10".to_string(), old.clone(), old]
    } else {
        [">=10 (reset top sub-comp)".to_string(), sub(), old]
    };

    List::new(list)
}

#[azptui::component]
pub fn sub() -> String {
    let counter = use_counter!();

    on_event!(|e: &Event| { e.is_key() }, |e: Event| {
        info!(
            "Got key {}. Get event handled!",
            e.as_key_event().unwrap().code
        )
    });

    format!("--> I am a sub-component 😎 : {}", counter)
}
