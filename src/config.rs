use crate::model::Command;
use pagurus::{
    event::KeyEvent,
    failure::{Failure, OrFail},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub key: KeyConfig,

    #[serde(default)]
    pub init: InitConfig,
}

impl Config {
    pub fn load_config_file() -> pagurus::Result<Option<Self>> {
        let Ok(home_dir) = std::env::var("HOME") else {
            return Ok(None);
        };

        let path = Path::new(&home_dir).join(".config").join("dotedit.json");
        if !path.exists() {
            return Ok(None);
        }

        let json = std::fs::read_to_string(&path)
            .or_fail()
            .map_err(|f| f.message(format!("Failed to read config file: {}", path.display())))?;
        serde_json::from_str(&json)
            .map_err(|e| {
                Failure::new().message(format!(
                    "Failed to parse config file: path={}, reason={e}",
                    path.display()
                ))
            })
            .map(Some)
    }
}

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(include_str!("../default-config.json")).expect("unreachable")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyCommand {
    Quit,
    #[serde(untagged)]
    Model(Command),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyConfig(BTreeMap<Key, KeyCommand>);

impl KeyConfig {
    pub fn get_command(&self, key: KeyEvent) -> Option<KeyCommand> {
        let key = Key(key);
        self.0.get(&key).cloned()
    }
}

impl Default for KeyConfig {
    fn default() -> Self {
        Config::default().key
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Key(KeyEvent);

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.ctrl {
            write!(f, "Ctrl+")?;
        }
        if self.0.alt {
            write!(f, "Alt+")?;
        }
        match self.0.key {
            pagurus::event::Key::Return => write!(f, "Enter"),
            pagurus::event::Key::Left => write!(f, "Left"),
            pagurus::event::Key::Right => write!(f, "Right"),
            pagurus::event::Key::Up => write!(f, "Up"),
            pagurus::event::Key::Down => write!(f, "Down"),
            pagurus::event::Key::Backspace => write!(f, "Backspace"),
            pagurus::event::Key::Delete => write!(f, "Delete"),
            pagurus::event::Key::Tab => write!(f, "Tab"),
            pagurus::event::Key::BackTab => write!(f, "BackTab"),
            pagurus::event::Key::Esc => write!(f, "Esc"),
            pagurus::event::Key::Char(c) => write!(f, "{}", c),
            _ => unreachable!(),
        }
    }
}

impl From<Key> for String {
    fn from(key: Key) -> Self {
        key.to_string()
    }
}

impl TryFrom<String> for Key {
    type Error = Failure;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut ctrl = false;
        let mut alt = false;
        let mut tokens = s.split('+').collect::<Vec<_>>();

        let last = tokens
            .pop()
            .or_fail()
            .map_err(|f| f.message("Empty key string"))?;
        let key = match last {
            "Enter" => pagurus::event::Key::Return,
            "Left" => pagurus::event::Key::Left,
            "Right" => pagurus::event::Key::Right,
            "Up" => pagurus::event::Key::Up,
            "Down" => pagurus::event::Key::Down,
            "Backspace" => pagurus::event::Key::Backspace,
            "Delete" => pagurus::event::Key::Delete,
            "Tab" => pagurus::event::Key::Tab,
            "BackTab" => pagurus::event::Key::BackTab,
            "Esc" => pagurus::event::Key::Esc,
            _ if last.chars().count() == 1 => match last.chars().next().or_fail()? {
                c @ ('a'..='z' | 'A'..='Z' | ' ') => pagurus::event::Key::Char(c),
                _ => return Err(Failure::new().message(format!("Unknown key: {last:?}"))),
            },
            _ => return Err(Failure::new().message(format!("Unknown key: {last:?}"))),
        };
        for token in tokens {
            match token {
                "Ctrl" => ctrl = true,
                "Alt" => alt = true,
                _ => return Err(Failure::new().message(format!("Unknown key modifier: {token:?}"))),
            }
        }

        Ok(Self(KeyEvent { key, ctrl, alt }))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitConfig(Vec<Command>);

impl InitConfig {
    pub fn commands(&self) -> &[Command] {
        &self.0
    }
}

impl Default for InitConfig {
    fn default() -> Self {
        Config::default().init
    }
}