use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestStatuses {
    #[default]
    Open,
    InJourney,
    Completed,
    Failed,
}

impl fmt::Display for QuestStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let quest_statuses = match self {
            QuestStatuses::Open => "Open",
            QuestStatuses::InJourney => "InJourney",
            QuestStatuses::Completed => "Completed",
            QuestStatuses::Failed => "Failed",
        };

        write!(f, "{}", quest_statuses)
    }
}
