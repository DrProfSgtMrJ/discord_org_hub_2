use poise::serenity_prelude::CreateModal;
use std::str::FromStr;

#[derive(Debug)]
pub enum Modal {
    CreateSeason,
}

impl Modal {
    pub fn title(&self) -> String {
        match self {
            Modal::CreateSeason => "Create Season".to_string(),
        }
    }

    pub fn id(&self) -> String {
        match self {
            Modal::CreateSeason => "season_create_modal".to_string(),
        }
    }
}

impl FromStr for Modal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "season_create_modal" => Ok(Modal::CreateSeason),
            _ => Err("Invalid modal ID".to_string()),
        }
    }
}

impl From<Modal> for CreateModal {
    fn from(modal: Modal) -> Self {
        match modal {
            Modal::CreateSeason => CreateModal::new(modal.id(), modal.title()),
        }
    }
}
