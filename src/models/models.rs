use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TodoData {
    pub entries: Vec<TodoEntry>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TodoEntry {
    pub title: String,
    pub state: TodoEntryState,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum TodoEntryState {
    #[default]
    InComplete,
    Completed,
}

impl TodoEntryState {
    pub fn to_indicator(&self) -> String {
        match self {
            Self::InComplete => String::from("□"),
            Self::Completed => String::from("✔"),
        }
    }
}
