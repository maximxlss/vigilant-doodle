use chrono::prelude::*;
use chrono::Duration as Dur;

pub fn format_datetime(dt: DateTime<Local>) -> String {
    let now = Local::now();
    let diff = now - dt;
    if diff < Dur::seconds(1) {
        "just now".to_string()
    } else if diff < Dur::minutes(1) {
        "seconds ago".to_string()
    } else if diff < Dur::days(1) {
        dt.format("at %H:%M").to_string()
    } else if dt.year() == now.year() {
        dt.format("at %H:%M %d/%m").to_string()
    } else {
        dt.format("at %H:%M %d/%m/%Y").to_string()
    }
}
