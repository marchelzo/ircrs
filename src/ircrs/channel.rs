use std::collections::HashSet;
use ircrs::text::Text;
use ircrs::msg::Message;

pub struct Channel {
    topic: String,
    nicks: HashSet<String>,
    msgs: Vec<Message>
}
