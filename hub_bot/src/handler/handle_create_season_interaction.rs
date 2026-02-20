use std::str::FromStr;

use chrono::NaiveDate;
use entity::sea_orm_active_enums::SeasonType;
use poise::serenity_prelude::{
    ActionRow, ActionRowComponent, Context, CreateActionRow, CreateInteractionResponseFollowup,
    ModalInteraction,
};

use crate::{Error, commands::SelectMenu};

use super::common::send_modal_error_response;
use crate::commands::InputText as InputTextComponent;
use service::{DbService, OrgService, SeasonService};

#[derive(Debug)]
struct SeasonFormData {
    title: String,
    num_players: String,
    start_date: String,
    end_date: Option<String>,
    season_type: Option<String>,
}

#[derive(Debug)]
struct SeasonParsedData {
    title: String,
    num_players: i32,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    season_type: Option<SeasonType>,
}

pub async fn handle_create_season_interaction(
    db_service: &DbService,
    ctx: &Context,
    interaction: &ModalInteraction,
) -> Result<(), Error> {
    interaction.defer(&ctx.http).await?;
    let components = &interaction.data.components;

    if let Ok(season_form_data) = extract_season_form_data(components) {
        if let Ok(parsed_season_data) = parse_season_form_data(&season_form_data) {
            if let Some(org_id) = interaction.guild_id {
                let org_discord_id = org_id.get().to_string();
                if let Some(org) = db_service
                    .get_org_by_discord_id(org_discord_id.as_str())
                    .await?
                {
                    let org_id = org.id;
                    match db_service
                        .create_season(
                            parsed_season_data.title.as_str(),
                            org_id,
                            parsed_season_data.num_players,
                            parsed_season_data.start_date,
                            parsed_season_data.end_date,
                            parsed_season_data.season_type,
                        )
                        .await
                    {
                        Ok(season) => {
                            // Reply with user select menu
                            send_add_members_to_season(
                                &season.id.to_string(),
                                &season.title,
                                ctx,
                                interaction,
                            )
                            .await?;
                        }
                        Err(_) => {
                            send_modal_error_response(ctx, interaction, "Failed to create season")
                                .await?;
                        }
                    }
                } else {
                    send_modal_error_response(
                        ctx,
                        interaction,
                        &format!(
                            "Failed to create season. Unable to find Org with ID {}",
                            org_id
                        ),
                    )
                    .await?;
                }
            }
        } else {
            send_modal_error_response(ctx, interaction, "Invalid form data").await?;
        }
    } else {
        send_modal_error_response(ctx, interaction, "Invalid form data").await?;
    }

    Ok(())
}

fn extract_season_form_data(components: &Vec<ActionRow>) -> Result<SeasonFormData, Error> {
    let mut title: Option<String> = None;
    let mut num_players: Option<String> = None;
    let mut start_date: Option<String> = None;
    let mut end_date: Option<String> = None;
    let mut season_type: Option<String> = None;

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
                    Ok(InputTextComponent::SeasonType) => {
                        if let Some(ref value) = input_text.value
                            && !value.trim().is_empty()
                        {
                            season_type = Some(value.to_lowercase().split_whitespace().collect())
                        }
                    }
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
        season_type,
    })
}

fn parse_season_form_data(season_form_data: &SeasonFormData) -> Result<SeasonParsedData, Error> {
    println!("Parsing season form data: {:?}", season_form_data);
    if let Ok(start_date) = NaiveDate::parse_from_str(&season_form_data.start_date, "%Y-%m-%d") {
        let end_date: Option<NaiveDate> = match &season_form_data.end_date {
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
        let num_players: i32 = season_form_data
            .num_players
            .parse()
            .map_err(|_| "Invalid number of players".to_string())?;
        let season_type = match &season_form_data.season_type {
            Some(season_type) => match SeasonType::from_str(season_type) {
                Ok(s) => Some(s),
                Err(e) => return Err(Error::from(format!("Invalid Season Type: {}", e))),
            },
            None => None,
        };

        Ok(SeasonParsedData {
            title: season_form_data.title.clone(),
            num_players,
            start_date,
            end_date,
            season_type,
        })
    } else {
        println!("Invalid start date");
        Err(Error::from("Invalid Start Date"))
    }
}

async fn send_add_members_to_season(
    season_uuid: &str,
    season_title: &str,
    ctx: &Context,
    interaction: &ModalInteraction,
) -> Result<(), Error> {
    let select_user_menu: CreateActionRow = SelectMenu::MemberSelectMenu {
        season_uuid: season_uuid.to_string(),
    }
    .into();
    interaction
        .create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new()
                .content(format!(
                    "Season: '{}' create successfully. Add members to season?",
                    season_title
                ))
                .components(vec![select_user_menu]),
        )
        .await?;

    Ok(())
}
