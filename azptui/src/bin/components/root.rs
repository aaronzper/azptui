use azptui_macros::use_counter;
use ratatui::widgets::List;

#[azptui::component]
pub fn root() -> List<'static> {
    let counter = use_counter!().to_string().clone();
    let list =
        List::new(["I am a component :)".to_string(), counter, sub(), sub()]);
    list
}

#[azptui::component]
pub fn sub() -> String {
    "--> I am a sub-component 😎".to_string()
}
