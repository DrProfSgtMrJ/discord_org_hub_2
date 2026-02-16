use std::str::FromStr;

use chrono::NaiveDate;
use poise::serenity_prelude::{
    ActionRow, ActionRowComponent, Context, CreateActionRow, CreateButton,
    CreateInteractionResponse, CreateInteractionResponseMessage, ModalInteraction,
};

use crate::Error;

use crate::commands::{Button, InputText as InputTextComponent};
use service::{DbService, OrgService, SeasonService};

#[derive(Debug)]
struct SeasonFormData {
    title: String,
    num_players: String,
    start_date: String,
    end_data: Option<String>,
}

#[derive(Debug)]
struct SeasonParsedData {
    title: String,
    num_players: i32,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
}

pub async fn handle_create_season_interaction(
    db_service: &DbService,
    ctx: &Context,
    interaction: &ModalInteraction,
) -> Result<(), Error> {
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
                        )
                        .await
                    {
                        Ok(season) => {
                            // Reply with button - asking if you want to set it as the org's current season
                            send_set_season_as_current_button(
                                &season.id.to_string(),
                                &season.title,
                                ctx,
                                interaction,
                            )
                            .await?;
                        }
                        Err(_) => {
                            send_error_response(ctx, interaction, "Failed to create season")
                                .await?;
                        }
                    }
                } else {
                    send_error_response(
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
            send_error_response(ctx, interaction, "Invalid form data").await?;
        }
    } else {
        send_error_response(ctx, interaction, "Invalid form data").await?;
    }

    Ok(())
}

async fn send_error_response(
    ctx: &Context,
    interaction: &ModalInteraction,
    error_message: &str,
) -> Result<(), Error> {
    match interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(error_message),
            ),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::from(err)),
    }
}

fn extract_season_form_data(components: &Vec<ActionRow>) -> Result<SeasonFormData, Error> {
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
        end_data: end_date,
    })
}

fn parse_season_form_data(season_form_data: &SeasonFormData) -> Result<SeasonParsedData, Error> {
    println!("Parsing season form data: {:?}", season_form_data);
    if let Ok(start_date) = NaiveDate::parse_from_str(&season_form_data.start_date, "%Y-%m-%d") {
        let end_date: Option<NaiveDate> = match &season_form_data.end_data {
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
            .map_err(|_| Error::from("Invalid number of players"))?;
        Ok(SeasonParsedData {
            title: season_form_data.title.clone(),
            num_players,
            start_date,
            end_date,
        })
    } else {
        println!("Invalid start date");
        Err(Error::from("Invalid Start Date"))
    }
}

async fn send_set_season_as_current_button(
    season_uuid: &str,
    season_title: &str,
    ctx: &Context,
    interaction: &ModalInteraction,
) -> Result<(), Error> {
    let yes_button = Button::SetAsCurrentSeasonYes {
        season_uuid: season_uuid.to_string(),
    };
    let no_button = Button::SetAsCurrentSeasonNo {
        season_uuid: season_uuid.to_string(),
    };
    let create_yes_button: CreateButton = yes_button.into();
    let create_no_button: CreateButton = no_button.into();
    let button_row = CreateActionRow::Buttons(vec![create_yes_button, create_no_button]);
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!(
                        "Season: '{}' created successfully. Set it as current season?",
                        season_title
                    ))
                    .components(vec![button_row]),
            ),
        )
        .await?;

    Ok(())
}
