use ircrs::channel::Channel;

pub enum Room {
    Channel(Channel),
    NoRoom
}
