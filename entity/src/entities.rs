use std::str::FromStr;

use crate::sea_orm_active_enums::SeasonType;

use super::discord_user::ActiveModel as DiscordUserActiveModel;
use super::member::ActiveModel as MemberActiveModel;
use super::org::ActiveModel as OrgActiveModel;
use super::season::ActiveModel as SeasonActiveModel;
use super::season_member::ActiveModel as SeasonMemberActiveModel;
use chrono::{NaiveDate, Utc};
use sea_orm::Set;
use uuid::Uuid;

impl DiscordUserActiveModel {
    pub fn new(
        discord_id: impl Into<String>,
        display_name: impl Into<String>,
        timezone: Option<impl Into<String>>,
        avatar_url: Option<impl Into<String>>,
    ) -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            discord_id: Set(discord_id.into()),
            display_name: Set(display_name.into()),
            timezone: Set(timezone.map_or(String::from("UTC"), Into::into)),
            avatar_url: Set(avatar_url.map_or(String::new(), Into::into)),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
    }
}

impl MemberActiveModel {
    pub fn new(discord_user_id: Uuid, org_id: Uuid) -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            user_id: Set(discord_user_id),
            org_id: Set(org_id),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
    }
}

impl OrgActiveModel {
    pub fn new(
        name: impl Into<String>,
        discord_id: impl Into<String>,
        owner_id: Uuid,
        current_season_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            name: Set(name.into()),
            discord_id: Set(discord_id.into()),
            owner_id: Set(owner_id),
            current_season_id: Set(current_season_id),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
    }
}

impl SeasonActiveModel {
    pub fn new(
        title: impl Into<String>,
        org_id: Uuid,
        num_players: i32,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        season_type: Option<SeasonType>,
    ) -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            title: Set(title.into()),
            org_id: Set(org_id),
            num_players: Set(num_players),
            start_date: Set(start_date),
            end_date: Set(end_date),
            season_type: Set(season_type.unwrap_or(SeasonType::Survivor)),
        }
    }
}

impl SeasonMemberActiveModel {
    pub fn new(season_id: Uuid, member_id: Uuid, placement: Option<i32>) -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            season_id: Set(season_id),
            member_id: Set(member_id),
            placement: Set(placement),
        }
    }
}

impl FromStr for SeasonType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Survivor" => Ok(SeasonType::Survivor),
            "Traitors" => Ok(SeasonType::Traitors),
            "BigBrother" => Ok(SeasonType::BigBrother),
            "TheChallenge" => Ok(SeasonType::TheChallenge),
            "Other" => Ok(SeasonType::Other),
            "survivor" => Ok(SeasonType::Survivor),
            "traitors" => Ok(SeasonType::Traitors),
            "bigbrother" => Ok(SeasonType::BigBrother),
            "thechallenge" => Ok(SeasonType::TheChallenge),
            "other" => Ok(SeasonType::Other),
            _ => Err("Invalid season type".to_string()),
        }
    }
}
