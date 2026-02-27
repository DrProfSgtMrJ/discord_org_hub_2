use poise::serenity_prelude::{CreateActionRow, CreateInputText, InputTextStyle};
use std::str::FromStr;

#[derive(Debug)]
#[warn(clippy::enum_variant_names)]
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

impl From<InputText> for CreateActionRow {
    fn from(input_text: InputText) -> Self {
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, input_text.title(), input_text.id())
                .placeholder(input_text.placeholder())
                .max_length(input_text.max_length())
                .required(input_text.required()),
        )
    }
}
