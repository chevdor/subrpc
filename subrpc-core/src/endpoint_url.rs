use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub enum EndpointUrl {
    Http(String),
    Https(String),
    Ws(String),
    Wss(String),
}

impl Display for EndpointUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndpointUrl::Http(s)
            | EndpointUrl::Https(s)
            | EndpointUrl::Ws(s)
            | EndpointUrl::Wss(s) => f.write_str(s),
        }
    }
}
