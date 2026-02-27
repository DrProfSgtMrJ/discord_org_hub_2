use poise::serenity_prelude::{ButtonStyle, CreateButton};
use std::str::FromStr;

#[derive(Debug)]
pub enum Button {
    SetAsCurrentSeasonYes { season_uuid: String },
    SetAsCurrentSeasonNo { season_uuid: String },
    RegisterOrgYes,
    RegisterOrgNo,
    MemberSelectSkip,
    SeasonsPrev,
    SeasonsNext,
    SeasonSurvivor { is_active: bool },
    SeasonTraitors { is_active: bool },
    SeasonBigBrother { is_active: bool },
    SeasonTheChallenge { is_active: bool },
    SeasonOther { is_active: bool },
}

impl Button {
    pub fn label(&self) -> String {
        match self {
            Button::SetAsCurrentSeasonYes { season_uuid: _ } => "Yes".to_string(),
            Button::SetAsCurrentSeasonNo { season_uuid: _ } => "No".to_string(),
            Button::RegisterOrgYes => "Yes".to_string(),
            Button::RegisterOrgNo => "No".to_string(),
            Button::MemberSelectSkip => "Skip".to_string(),
            Button::SeasonsPrev => "◀".to_string(),
            Button::SeasonsNext => "▶".to_string(),
            Button::SeasonSurvivor { .. } => "🏝️ Survivor".to_string(),
            Button::SeasonTraitors { .. } => "🔪 Traitors".to_string(),
            Button::SeasonBigBrother { .. } => "👁️ Big Brother".to_string(),
            Button::SeasonTheChallenge { .. } => "🏆 The Challenge".to_string(),
            Button::SeasonOther { .. } => "Other".to_string(),
        }
    }

    pub fn style(&self) -> ButtonStyle {
        match self {
            Button::SetAsCurrentSeasonYes { season_uuid: _ } => ButtonStyle::Primary,
            Button::SetAsCurrentSeasonNo { season_uuid: _ } => ButtonStyle::Secondary,
            Button::RegisterOrgYes => ButtonStyle::Primary,
            Button::RegisterOrgNo => ButtonStyle::Secondary,
            Button::SeasonsPrev => ButtonStyle::Secondary,
            Button::SeasonsNext => ButtonStyle::Secondary,
            Button::MemberSelectSkip => ButtonStyle::Secondary,
            Button::SeasonSurvivor { is_active } => {
                if *is_active {
                    ButtonStyle::Primary
                } else {
                    ButtonStyle::Secondary
                }
            }
            Button::SeasonTraitors { is_active } => {
                if *is_active {
                    ButtonStyle::Primary
                } else {
                    ButtonStyle::Secondary
                }
            }
            Button::SeasonBigBrother { is_active } => {
                if *is_active {
                    ButtonStyle::Primary
                } else {
                    ButtonStyle::Secondary
                }
            }
            Button::SeasonTheChallenge { is_active } => {
                if *is_active {
                    ButtonStyle::Primary
                } else {
                    ButtonStyle::Secondary
                }
            }
            Button::SeasonOther { is_active } => {
                if *is_active {
                    ButtonStyle::Primary
                } else {
                    ButtonStyle::Secondary
                }
            }
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
            Button::RegisterOrgYes => "register_org_yes".to_string(),
            Button::RegisterOrgNo => "register_org_no".to_string(),
            Button::SeasonsPrev => "seasons_prev".to_string(),
            Button::SeasonsNext => "seasons_next".to_string(),
            Button::MemberSelectSkip => "member_select_skip".to_string(),
            Button::SeasonSurvivor { is_active } => {
                format!("seasons_filter:Survivor:{}", is_active)
            }
            Button::SeasonTraitors { is_active } => {
                format!("seasons_filter:Traitors:{}", is_active)
            }
            Button::SeasonBigBrother { is_active } => {
                format!("seasons_filter:BigBrother:{}", is_active)
            }
            Button::SeasonTheChallenge { is_active } => {
                format!("seasons_filter:TheChallenge:{}", is_active)
            }
            Button::SeasonOther { is_active } => format!("seasons_filter:Other:{}", is_active),
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
        let action = parts.first().ok_or("Invalid")?;
        match *action {
            "register_org_no" => Ok(Button::RegisterOrgNo),
            "set_as_current_season_yes" => Ok(Button::SetAsCurrentSeasonYes {
                season_uuid: parts.get(1).ok_or("Invalid")?.to_string(),
            }),
            "set_as_current_season_no" => Ok(Button::SetAsCurrentSeasonNo {
                season_uuid: parts.get(1).ok_or("Invalid")?.to_string(),
            }),
            "register_org_yes" => Ok(Button::RegisterOrgYes),
            "seasons_prev" => Ok(Button::SeasonsPrev),
            "seasons_next" => Ok(Button::SeasonsNext),
            "member_select_skip" => Ok(Button::MemberSelectSkip),
            "seasons_filter" => match parts.get(1).ok_or("Invalid") {
                Ok(ty) => {
                    let is_active = parts
                        .get(2)
                        .ok_or("Invalid".to_string())?
                        .parse::<bool>()
                        .map_err(|_| "Invalid".to_string())?;
                    match *ty {
                        "Survivor" => Ok(Button::SeasonSurvivor { is_active }),
                        "BigBrother" => Ok(Button::SeasonBigBrother { is_active }),
                        "Traitors" => Ok(Button::SeasonTraitors { is_active }),
                        "TheChallenge" => Ok(Button::SeasonTheChallenge { is_active }),
                        "Other" => Ok(Button::SeasonOther { is_active }),
                        _ => Err("Invalid filter type".to_string()),
                    }
                }
                Err(e) => Err(e.to_string()),
            },
            _ => Err("Invalid action".to_string()),
        }
    }
}
