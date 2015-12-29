#![feature(slice_patterns)]
#![feature(convert)]

extern crate time;

mod ircrs {
    pub mod session;
    pub mod config;
    pub mod channel;
    pub mod text;
    pub mod msg;
    pub mod room;
}

use std::net::TcpStream;
use ircrs::session::Session;
use ircrs::config::Config;

fn start(session: Session) {
    println!("Connected!");
}

fn main() {
    let cfg  = Config::load("./cfg");
    let conn = TcpStream::connect("irc.freenode.net:6667");
    match (cfg, conn) {
        (Ok(config), Ok(stream)) => Session::new(stream, config).run(),
        (Err(e), _)              => println!("Failed to load configuration file: {}.", e),
        (_, Err(e))              => println!("Failed to connect to the IRC server: {}.", e)
    }
}
