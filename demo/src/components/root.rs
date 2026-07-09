use azptui::use_counter;
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
    format!("--> I am a sub-component 😎 : {}", counter)
}
