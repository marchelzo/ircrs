use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

pub struct Config {
    pub nick: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new(nick: String, username: String, password: String) -> Config {
        return Config {
            nick: nick,
            username: username,
            password: password
        };
    }

    /*
     * The configuration file should be of the form:
     *
     * *(<option> <value> \n)
     *
     * e.g.,
     *
     * nick marchelzo
     * username marchelzo
     * password foobarbaz
     */
    pub fn load(path: &'static str) -> Result<Config, String> {
        let f =  File::open(path);

        if let Err(e) = f {
            return Err(e.to_string());
        }

        let mut hm: HashMap<String, String> = HashMap::new();

        for line in BufReader::new(f.unwrap()).lines() {
            if let Err(e) = line {
                return Err(e.to_string());
            }

            let line = line.unwrap();
            let words: Vec<&str>  = line.split_whitespace().collect();
            match words.as_slice() {
                [key, value] => { hm.insert(key.to_string(), value.to_string()); },
                _            => return Err("invalid syntax".to_string())
            }
        }

        return match (hm.remove("nick"), hm.remove("username"), hm.remove("password")) {
            (Some(nick), Some(username), Some(password)) => Ok(Config::new(nick, username, password)),
            _                                            => Err("nick/username/password missing".to_string())
        };
    }
}
