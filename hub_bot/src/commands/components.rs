use std::str::FromStr;

use poise::serenity_prelude::{
    ButtonStyle, CreateActionRow, CreateButton, CreateInputText, CreateModal, CreateSelectMenu,
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

#[derive(Debug)]
pub enum Button {
    SetAsCurrentSeasonYes { season_uuid: String },
    SetAsCurrentSeasonNo { season_uuid: String },
}

impl Button {
    pub fn label(&self) -> String {
        match self {
            Button::SetAsCurrentSeasonYes { season_uuid: _ } => "Yes".to_string(),
            Button::SetAsCurrentSeasonNo { season_uuid: _ } => "No".to_string(),
        }
    }

    pub fn style(&self) -> ButtonStyle {
        match self {
            Button::SetAsCurrentSeasonYes { season_uuid: _ } => ButtonStyle::Primary,
            Button::SetAsCurrentSeasonNo { season_uuid: _ } => ButtonStyle::Secondary,
        }
    }

    pub fn id(&self) -> String {
        match self {
            Button::SetAsCurrentSeasonYes { season_uuid } => {
                format!("set_as_current_season_yes:{}", season_uuid)
            }
            Button::SetAsCurrentSeasonNo { season_uuid } => {
                format!("set_as_current_season_no:{}", season_uuid)
            }
        }
        .to_string()
    }
}

impl From<Button> for CreateButton {
    fn from(button: Button) -> Self {
        CreateButton::new(button.id())
            .style(button.style())
            .label(button.label())
            .custom_id(button.id())
    }
}

impl FromStr for Button {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // extract season_uuid from s
        let parts = s.split(":").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("Invalid format".to_string());
        }
        let action = parts.first().ok_or("Invalid")?;
        let param = parts.get(1).ok_or("Invalid")?;
        match *action {
            "set_as_current_season_yes" => Ok(Button::SetAsCurrentSeasonYes {
                season_uuid: param.to_string(),
            }),
            "set_as_current_season_no" => Ok(Button::SetAsCurrentSeasonNo {
                season_uuid: param.to_string(),
            }),
            _ => Err("Invalid action".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum SelectMenu {
    SeasonSelectMenu(Vec<entity::season::Model>),
    MemberSelectMenu(Vec<entity::member::Model>),
}

impl SelectMenu {
    pub fn id(&self) -> String {
        match self {
            SelectMenu::SeasonSelectMenu(_) => "season_select_menu".to_string(),
            SelectMenu::MemberSelectMenu(_) => "member_select_menu".to_string(),
        }
    }

    pub fn placeholder(&self) -> &str {
        match self {
            SelectMenu::SeasonSelectMenu(_) => "Select Season to register members to...",
            SelectMenu::MemberSelectMenu(_) => "Select Members to register to...",
        }
    }

    pub fn options(&self) -> Vec<CreateSelectMenuOption> {
        match self {
            SelectMenu::SeasonSelectMenu(seasons) => seasons
                .iter()
                .map(|season| {
                    CreateSelectMenuOption::new(
                        format!("{} (Started: {:?})", season.title, season.start_date),
                        season.id,
                    )
                })
                .collect(),
            SelectMenu::MemberSelectMenu(members) => members
                .iter()
                .map(|member| CreateSelectMenuOption::new(format!("{}", member.id), member.id))
                .collect(),
        }
    }

    pub fn kind(&self) -> CreateSelectMenuKind {
        match self {
            SelectMenu::SeasonSelectMenu(_) => CreateSelectMenuKind::String {
                options: self.options(),
            },
            SelectMenu::MemberSelectMenu(_) => CreateSelectMenuKind::String {
                options: self.options(),
            },
        }
    }

    pub fn min_values(&self) -> u8 {
        match self {
            SelectMenu::SeasonSelectMenu(_) => 1,
            SelectMenu::MemberSelectMenu(_) => 1,
        }
    }

    pub fn max_values(&self) -> u8 {
        match self {
            SelectMenu::SeasonSelectMenu(_) => 1,
            SelectMenu::MemberSelectMenu(_) => 1,
        }
    }
}

impl From<SelectMenu> for CreateSelectMenu {
    fn from(select_menu: SelectMenu) -> Self {
        CreateSelectMenu::new(select_menu.id(), select_menu.kind())
            .placeholder(select_menu.placeholder())
            .min_values(select_menu.min_values())
            .max_values(select_menu.max_values())
    }
}
