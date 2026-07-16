use azptui::{on_event, use_state};
use crossterm::event::KeyCode;
use ratatui::widgets::List;

#[azptui::component]
pub fn root() -> List<'static> {
    let (entered, set_entered) = use_state!(false);
    let (typed, set_typed) = use_state!(String::new());

    on_event!(
        |e| e.is_key() && e.as_key_event().unwrap().code == KeyCode::Esc,
        // this is hacky but the only clean way i can think to do it rn
        |_| panic!("Exit"),
    );

    on_event!(
        |e| e.is_key() && e.as_key_event().unwrap().code == KeyCode::Enter,
        move |_| set_entered(true),
    );

    let typed_copy = typed.clone();
    on_event!(
        |e| e.is_key()
            && matches!(e.as_key_event().unwrap().code, KeyCode::Char(_)),
        move |e| {
            let c = e.as_key_event().unwrap().code.as_char().unwrap();
            set_typed(format!("{}{}", typed_copy, c));
        }
    );

    let old = sub();

    let list = if !entered {
        [
            "Press enter to reset the top sub-component (by spawning a new one)...".
                to_string(),
            old.clone(),
            old,
            typed,
        ]
    } else {
        [
            "Good boy. Press Esc to exit.".to_string(),
            sub(),
            old,
            typed,
        ]
    };

    List::new(list)
}

#[azptui::component]
pub fn sub() -> String {
    let (key, set_key) = use_state!(KeyCode::Null);
    let (count, set_count) = use_state!(0);

    on_event!(|e| { e.is_key() }, move |e| {
        set_count(count + 1);
        set_key(e.as_key_event().unwrap().code);
    });

    format!("{} --> {:?}", count, key)
}
