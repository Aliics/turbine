extern crate serde;
extern crate toml;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: Server,
    pub web: Web,
}

#[allow(dead_code)]
impl Config {
    fn ser(&self) -> String {
        toml::to_string(self).unwrap()
    }

    pub fn from(ser: String) -> Config {
        toml::from_str::<Config>(&ser).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Web {
    pub http: String,
    pub directory: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn config_deserialize() {
        let some_toml = given_example_conf();
        let actual_conf = Config::from(some_toml);
        let expect_conf = given_conf();

        assert_eq!(actual_conf.server.address, expect_conf.server.address);
        assert_eq!(actual_conf.web.http, expect_conf.web.http);
        assert_eq!(actual_conf.web.directory, expect_conf.web.directory);
    }

    #[test]
    fn config_serialize() {
        let actual_conf = given_conf().ser();
        let expected_toml = given_example_conf();

        assert_eq!(actual_conf.trim(), expected_toml.trim());
    }

    fn given_example_conf() -> String {
        fs::read_to_string("tests/example_conf.toml").unwrap()
    }

    fn given_conf() -> Config {
        Config {
            server: Server {
                address: "localhost:8080".to_string(),
            },
            web: Web {
                http: "1.1".to_string(),
                directory: "${HOME}/development/turbine/tests/http".to_string(),
            },
        }
    }
}
