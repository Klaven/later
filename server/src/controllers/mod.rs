use rocket::{Route};
pub mod message;

pub fn get_routes() -> Vec<Route> {
    routes![message::get_messages,
            message::post_message]
}
