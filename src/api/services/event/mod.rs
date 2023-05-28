mod create_event;
mod event_list;
mod remove_event;
mod types;
mod update_event;

use actix_web::{web, Scope};

pub fn event_scope() -> Scope {
    web::scope("/event")
        .service(event_list::event_list)
        .service(create_event::create_event)
        .service(remove_event::remove_event)
}
