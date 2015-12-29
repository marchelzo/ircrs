use ircrs::text::Text;
use time::SteadyTime;

pub struct Message {
    time: SteadyTime,
    source: Text,
    body: Vec<Text>
}
