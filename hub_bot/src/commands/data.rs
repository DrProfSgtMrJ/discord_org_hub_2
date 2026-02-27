use crate::Error;
use crate::components::InputText as InputTextComponent;
use chrono::NaiveDate;
use poise::serenity_prelude::{ActionRow, ActionRowComponent};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonFormData {
    pub title: String,
    pub num_players: String,
    pub start_date: String,
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonParsedData {
    pub title: String,
    pub num_players: i32,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

impl SeasonFormData {
    pub fn from_action_rows(components: &Vec<ActionRow>) -> Result<Self, Error> {
        let mut title: Option<String> = None;
        let mut num_players: Option<String> = None;
        let mut start_date: Option<String> = None;
        let mut end_date: Option<String> = None;

        for row in components {
            for component in &row.components {
                if let ActionRowComponent::InputText(input_text) = component {
                    match InputTextComponent::from_str(&input_text.custom_id) {
                        Ok(InputTextComponent::SeasonEndDate) => {
                            if let Some(ref value) = input_text.value
                                && !value.trim().is_empty()
                            {
                                end_date = Some(value.clone())
                            }
                        }
                        Ok(InputTextComponent::SeasonNumPlayers) => {
                            num_players = input_text.value.clone()
                        }
                        Ok(InputTextComponent::SeasonStartDate) => {
                            start_date = input_text.value.clone()
                        }
                        Ok(InputTextComponent::SeasonTitle) => title = input_text.value.clone(),
                        Err(e) => return Err(Error::from(e)),
                    }
                }
            }
        }

        Ok(SeasonFormData {
            title: title.ok_or(Error::from("Title is required"))?,
            num_players: num_players.ok_or(Error::from("Num Players is required"))?,
            start_date: start_date.ok_or(Error::from("Start Date is required"))?,
            end_date,
        })
    }

    pub fn parsed_data(&self) -> Result<SeasonParsedData, Error> {
        if let Ok(start_date) = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d") {
            let end_date: Option<NaiveDate> = match &self.end_date {
                Some(end_date) => match NaiveDate::parse_from_str(end_date, "%Y-%m-%d") {
                    Ok(date) => {
                        if start_date > date {
                            println!("End date must be after start date");
                            return Err(Error::from("End Date must be after Start Date"));
                        }
                        Some(date)
                    }
                    Err(_) => return Err(Error::from("Invalid End Date")),
                },
                None => None,
            };
            let num_players: i32 = self
                .num_players
                .parse()
                .map_err(|_| "Invalid number of players".to_string())?;

            Ok(SeasonParsedData {
                title: self.title.clone(),
                num_players,
                start_date,
                end_date,
            })
        } else {
            println!("Invalid start date");
            Err(Error::from("Invalid Start Date"))
        }
    }
}

