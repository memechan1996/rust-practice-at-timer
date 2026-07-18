use inquire_derive::Selectable;
use std::fmt;

#[derive(Debug, Copy, Clone, Selectable)]
pub enum Command {
    Start,
    Exit,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Command::Start => "Start",
            Command::Exit => "Exit",
        };
        write!(f, "{}", s)
    }
}
