use poise::serenity_prelude::{
    CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};
use std::str::FromStr;

use entity::sea_orm_active_enums::SeasonType;
use sea_orm::{ActiveEnum, Iterable};

#[derive(Debug)]
pub enum SelectMenu {
    MemberSelectMenu { season_uuid: String },
    SeasonTypeSelectMenu,
}

impl SelectMenu {
    pub fn id(&self) -> String {
        match self {
            SelectMenu::MemberSelectMenu { season_uuid } => {
                format!("member_select_menu:{}", season_uuid)
            }
            SelectMenu::SeasonTypeSelectMenu => "season_type_select_menu".to_string(),
        }
    }

    pub fn placeholder(&self) -> &str {
        match self {
            SelectMenu::MemberSelectMenu { .. } => "Select Members to register to...",
            SelectMenu::SeasonTypeSelectMenu => "Select Season Type...",
        }
    }

    pub fn options(&self) -> Vec<CreateSelectMenuOption> {
        match self {
            SelectMenu::MemberSelectMenu { .. } => vec![],
            SelectMenu::SeasonTypeSelectMenu => SeasonType::iter()
                .map(|season_type| {
                    CreateSelectMenuOption::new(season_type.to_value(), season_type.to_value())
                })
                .collect(),
        }
    }

    pub fn kind(&self) -> CreateSelectMenuKind {
        match self {
            SelectMenu::MemberSelectMenu { .. } => CreateSelectMenuKind::User {
                default_users: None,
            },
            SelectMenu::SeasonTypeSelectMenu => CreateSelectMenuKind::String {
                options: self.options(),
            },
        }
    }

    pub fn min_values(&self) -> u8 {
        match self {
            SelectMenu::MemberSelectMenu { .. } => 0,
            SelectMenu::SeasonTypeSelectMenu => 1,
        }
    }

    pub fn max_values(&self) -> u8 {
        match self {
            SelectMenu::MemberSelectMenu { .. } => 20,
            SelectMenu::SeasonTypeSelectMenu => 1,
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

impl From<SelectMenu> for CreateActionRow {
    fn from(select_menu: SelectMenu) -> Self {
        CreateActionRow::SelectMenu(select_menu.into())
    }
}

impl FromStr for SelectMenu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<&str>>();
        let action = parts.first().ok_or("Invalid")?;
        match *action {
            "member_select_menu" => Ok(SelectMenu::MemberSelectMenu {
                season_uuid: parts.get(1).ok_or("Invalid")?.to_string(),
            }),
            "season_type_select_menu" => Ok(SelectMenu::SeasonTypeSelectMenu),
            _ => Err("Invalid select menu ID".to_string()),
        }
    }
}
