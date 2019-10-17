use rocket::http::RawStr;

#[get("/api/v1/message/<name>")]
pub fn get_messages(name: &RawStr) ->  String {
    format!("Sup {}!", name.as_str())
}

#[post("/api/v1/message?wave&<msg>")]
pub fn post_message(msg: &RawStr) -> String {
    format!("your message: {}", msg.as_str())
}
