use std::rc::Rc;

use azptui::{on_event, use_state};
use crossterm::event::KeyCode;
use log::info;
use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

/// The full state of the calculator, kept in a single `use_state!` so every
/// event handler can atomically replace it.
#[derive(Clone, Default)]
struct Calc {
    /// Running total (left-hand side of the pending operation).
    acc: f64,
    /// Operator waiting for a right-hand side, if any.
    pending: Option<char>,
    /// The number currently being typed.
    entry: String,
    /// True right after `=`, so the next digit starts a fresh calculation.
    evaluated: bool,
}

fn apply(lhs: f64, op: char, rhs: f64) -> f64 {
    match op {
        '+' => lhs + rhs,
        '-' => lhs - rhs,
        '*' => lhs * rhs,
        '/' => lhs / rhs,
        _ => unreachable!(),
    }
}

fn fmt_num(n: f64) -> String {
    if n == n.trunc() && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

#[azptui::component]
pub fn calculator() -> Paragraph<'static> {
    let (calc, set_calc) = use_state!(Calc::default());
    // The setter isn't Clone, but several handlers below need it, so share
    // it behind an Rc.
    let set_calc = Rc::new(set_calc);

    on_event!(
        |e| e.is_key() && e.as_key_event().unwrap().code == KeyCode::Esc,
        // this is hacky but the only clean way i can think to do it rn
        |_| panic!("Exit"),
    );

    // Digits and the decimal point extend the current entry
    let cur = calc.clone();
    let set = Rc::clone(&set_calc);
    on_event!(
        |e| e.is_key()
            && matches!(
                e.as_key_event().unwrap().code,
                KeyCode::Char(c) if c.is_ascii_digit() || c == '.'
            ),
        move |e| {
            let ch = e.as_key_event().unwrap().code.as_char().unwrap();
            let mut c = if cur.evaluated {
                Calc::default()
            } else {
                cur.clone()
            };
            if ch != '.' || !c.entry.contains('.') {
                c.entry.push(ch);
            }
            set(c);
        }
    );

    // Operators fold the entry into the accumulator and start a new entry
    let cur = calc.clone();
    let set = Rc::clone(&set_calc);
    on_event!(
        |e| e.is_key()
            && matches!(
                e.as_key_event().unwrap().code,
                KeyCode::Char('+' | '-' | '*' | '/')
            ),
        move |e| {
            let op = e.as_key_event().unwrap().code.as_char().unwrap();
            let mut c = cur.clone();
            if !c.entry.is_empty() {
                let rhs = c.entry.parse().unwrap_or(0.0);
                c.acc = match c.pending {
                    Some(p) => apply(c.acc, p, rhs),
                    None => rhs,
                };
                c.entry.clear();
            }
            c.pending = Some(op);
            c.evaluated = false;
            info!("Pending: {} {}", fmt_num(c.acc), op);
            set(c);
        }
    );

    // Enter or '=' evaluates the pending operation
    let cur = calc.clone();
    let set = Rc::clone(&set_calc);
    on_event!(
        |e| e.is_key() && {
            let code = e.as_key_event().unwrap().code;
            code == KeyCode::Enter || code == KeyCode::Char('=')
        },
        move |_| {
            let c = cur.clone();
            if let Some(op) = c.pending {
                let rhs = c.entry.parse().unwrap_or(0.0);
                let result = apply(c.acc, op, rhs);
                info!(
                    "{} {} {} = {}",
                    fmt_num(c.acc),
                    op,
                    fmt_num(rhs),
                    fmt_num(result)
                );
                set(Calc {
                    acc: result,
                    pending: None,
                    entry: fmt_num(result),
                    evaluated: true,
                });
            }
        }
    );

    // Backspace edits the entry (or wipes a finished result)
    let cur = calc.clone();
    let set = Rc::clone(&set_calc);
    on_event!(
        |e| e.is_key() && e.as_key_event().unwrap().code == KeyCode::Backspace,
        move |_| {
            let mut c = cur.clone();
            if c.evaluated {
                c = Calc::default();
            } else {
                c.entry.pop();
            }
            set(c);
        }
    );

    // 'c' clears everything
    let set = Rc::clone(&set_calc);
    on_event!(
        |e| e.is_key()
            && matches!(
                e.as_key_event().unwrap().code,
                KeyCode::Char('c' | 'C')
            ),
        move |_| {
            info!("Cleared");
            set(Calc::default());
        }
    );

    let expression = match calc.pending {
        Some(op) => format!("{} {}", fmt_num(calc.acc), op),
        None => String::new(),
    };
    let entry = if calc.entry.is_empty() {
        "0".to_string()
    } else {
        calc.entry
    };

    let text = Text::from(vec![
        Line::from(expression).dim().right_aligned(),
        Line::from(entry).bold().right_aligned(),
        Line::default(),
        Line::from(activity()).dim().centered(),
    ]);

    Paragraph::new(text).block(
        Block::bordered().title("Calculator").title_bottom(
            Line::from(
                "0-9 .  + - * /  =/Enter  Backspace  c: clear  Esc: quit",
            )
            .centered()
            .dim(),
        ),
    )
}

/// Sub-component with its own state: a running tally of keypresses.
#[azptui::component]
fn activity() -> String {
    let (last, set_last) = use_state!(String::from("none yet"));
    let (count, set_count) = use_state!(0u32);

    on_event!(|e| e.is_key(), move |e| {
        set_count(count + 1);
        set_last(format!("{:?}", e.as_key_event().unwrap().code));
    });

    format!("keys pressed: {count}   last: {last}")
}
