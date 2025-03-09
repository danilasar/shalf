use crate::app::App;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct FileContent {
    program_url: Option<String>
}

pub struct ParseError(String);

impl App {
    pub fn parse_slft(content: &str) -> Result<Self, ParseError> { 
        let file : Result<FileContent, toml::de::Error> = toml::from_str(content);
        if file.is_err() {
            return Err(ParseError("toml parse error".into()));
        }
        let file = file.unwrap();
        if file.program_url.is_none() {
            return Err(ParseError("no program_url".into()));
        }
        Self {
            program: vec![]
        }
    }

    pub fn parse_slf(_: &[u8]) -> Result<Self, ParseError> {
        Err(ParseError("not implemented".into()))
    }
}
