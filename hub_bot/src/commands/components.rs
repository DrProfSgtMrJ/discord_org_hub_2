use std::str::FromStr;

use poise::serenity_prelude::{
    ChannelType, CreateActionRow, CreateInputText, CreateModal, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption, InputTextStyle,
};

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
        "season_create_modal".to_string()
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

impl Into<CreateModal> for Modal {
    fn into(self) -> CreateModal {
        match self {
            Modal::CreateSeason => CreateModal::new(self.id(), self.title()),
        }
    }
}

#[derive(Debug)]
pub enum InputText {
    SeasonTitle,
    SeasonNumPlayers,
    SeasonStartDate,
    SeasonEndDate,
}

impl InputText {
    pub fn title(&self) -> String {
        match self {
            InputText::SeasonTitle => "Season Title".to_string(),
            InputText::SeasonNumPlayers => "Number of Players".to_string(),
            InputText::SeasonStartDate => "Start Date".to_string(),
            InputText::SeasonEndDate => "End Date".to_string(),
        }
    }

    pub fn id(&self) -> String {
        match self {
            InputText::SeasonTitle => "season_title".to_string(),
            InputText::SeasonNumPlayers => "season_num_players".to_string(),
            InputText::SeasonStartDate => "season_start_date".to_string(),
            InputText::SeasonEndDate => "season_end_date".to_string(),
        }
    }

    pub fn placeholder(&self) -> String {
        match self {
            InputText::SeasonTitle => {
                "Enter season title (e.g., 'Pokemon Extravaganza!')".to_string()
            }
            InputText::SeasonNumPlayers => "Enter number of players".to_string(),
            InputText::SeasonStartDate => "Enter start date (YYYY-MM-DD)".to_string(),
            InputText::SeasonEndDate => {
                "Enter end date (YYYY-MM-DD) - Leave empty for no end date".to_string()
            }
        }
    }

    pub fn max_length(&self) -> u16 {
        match self {
            InputText::SeasonTitle => 100,
            InputText::SeasonNumPlayers => 3,
            InputText::SeasonStartDate => 10,
            InputText::SeasonEndDate => 10,
        }
    }

    pub fn required(&self) -> bool {
        match self {
            InputText::SeasonTitle => true,
            InputText::SeasonNumPlayers => true,
            InputText::SeasonStartDate => true,
            InputText::SeasonEndDate => false,
        }
    }
}

impl FromStr for InputText {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "season_title" => Ok(InputText::SeasonTitle),
            "season_num_players" => Ok(InputText::SeasonNumPlayers),
            "season_start_date" => Ok(InputText::SeasonStartDate),
            "season_end_date" => Ok(InputText::SeasonEndDate),
            _ => Err("Invalid input text ID".to_string()),
        }
    }
}

impl Into<CreateActionRow> for InputText {
    fn into(self) -> CreateActionRow {
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, self.title(), self.id())
                .placeholder(self.placeholder())
                .max_length(self.max_length())
                .required(self.required()),
        )
    }
}
