pub enum Message {
    Question,
    Silent,
    Shouting,
    Anything,
}

pub fn reply<'a>(dialouge: &'a str) -> &'a str {
    if dialouge.is_empty() {
        observe_dialog(&Message::Silent)
    } else if dialouge == dialouge.to_uppercase() {
        observe_dialog(&Message::Shouting)
    } else if dialouge.ends_with('?') {
        observe_dialog(&Message::Anything)
    } else {
        observe_dialog(&Message::Question)
    }
}

fn observe_dialog<'a>(msg: &Message) -> &'a str {
    match *msg {
        Message::Question => "Whatever.",
        Message::Shouting => "Whoa, chill out!",
        Message::Silent => "Fine. Be that way!",
        Message::Anything => "Sure.",
    }
}
