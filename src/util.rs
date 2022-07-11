use chrono::{DateTime, Local, NaiveDateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::Event;

pub fn get_unix_time() -> i64 {
    let t = SystemTime::now();
    t.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

pub fn get_seconds_before_midnight(ts: i64) -> i64 {
    86400 - ts % 86400
}

pub fn format_ts(ts: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(ts, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let local: DateTime<Local> = datetime.into();
    local.format("%d.%m %H:%M").to_string()
}

pub fn format_event_title(event: &Event) -> String {
    if event.link.len() > 0 {
        format!(
            "<a href=\"{}\">{}</a>",
            event.link,
            event.name,
        )
    } else {
        event.name.to_string()
    }
}

#[test]
fn test_util() {
    assert_eq!(format_ts(1650445814), "20.04 11:10");
    assert_eq!(get_seconds_before_midnight(1651503600), 9 * 60 * 60);
}
