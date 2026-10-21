use chrono::Weekday;
use leptos::IntoView;
use leptos::html::{ElementChild, div};
use leptos::prelude::ClassAttribute;
use leptos_ui::clx::tw_merge;
use models::cal::GeneralTime;
use models::cal::time_cells::common::{Rule, Status};
use models::cal::time_cells::weekly::WeeklyTimeCell;

pub fn basic_cell(status: Status, div_cap: u16, rules: Vec<Rule>) -> impl IntoView {
    let label = if rules.is_empty() {
        "No Rules".to_owned()
    } else {
        format!("{} Rules", rules.len())
    };
    let annotate = match div_cap {
        0 => "∞".to_owned(),
        1 => "1".to_owned(),
        _ => format!("1 / {}", div_cap),
    };
    div()
        .class(tw_merge!(
            r#"relative flex items-center justify-center rounded-sm border-transparent
            w-full h-full border border-border hover:border-primary"#,
            get_theme(&status)
        ))
        .child((
            div().class("text-xs").child(label),
            div()
                .class("absolute bottom-0 right-1 text-[8px]")
                .child(annotate),
        ))
}

pub fn none_cell(/* week_day: &Weekday, time_cells: &Vec<WeeklyTimeCell<GeneralTime>> */) -> impl IntoView {
    div().class("h-full bg-none flex-1 border border-border/0 hover:border-primary")
        // .child(time_cells.iter())
}

fn get_theme(status: &Status) -> &'static str {
    match status {
        Status::Available => "bg-avail text-avail-foreground",
        Status::Unavailable => "bg-unavail text-unavail-foreground",
        Status::Preferred => "bg-pref text-pref-foreground",
        Status::Undesired => "bg-undesired text-undesired-foreground",
    }
}