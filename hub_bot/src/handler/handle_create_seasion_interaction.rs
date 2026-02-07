use std::str::FromStr;

use poise::serenity_prelude::{
    ActionRow, ActionRowComponent, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, Error, ModalInteraction,
};

use crate::commands::InputText;
use service::DbService;

#[derive(Debug)]
struct SeasonFormData {
    title: String,
    num_players: String,
    start_date: String,
    end_data: Option<String>,
}

pub async fn handle_create_season_interaction(
    db_service: &DbService,
    ctx: &Context,
    interaction: &ModalInteraction,
) -> Result<(), Error> {
    let components = &interaction.data.components;

    if let Ok(season_form_data) = extract_season_form_data(components) {
        println!("SeasonFormData: {:?}", season_form_data);
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
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(error_message),
            ),
        )
        .await
}

fn extract_season_form_data(components: &Vec<ActionRow>) -> Result<SeasonFormData, Error> {
    let mut title: Option<String> = None;
    let mut num_players: Option<String> = None;
    let mut start_date: Option<String> = None;
    let mut end_date: Option<String> = None;

    for row in components {
        for component in &row.components {
            match component {
                ActionRowComponent::InputText(input_text) => {
                    // Handle input text component
                    if let Ok(input) = InputText::from_str(&input_text.custom_id.as_str()) {
                        match input {
                            InputText::SeasonTitle => title = input_text.value.clone(),
                            InputText::SeasonNumPlayers => num_players = input_text.value.clone(),
                            InputText::SeasonStartDate => start_date = input_text.value.clone(),
                            InputText::SeasonEndDate => end_date = input_text.value.clone(),
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(SeasonFormData {
        title: title.ok_or(Error::Other("Title is required"))?,
        num_players: num_players.ok_or(Error::Other("Num Players is required"))?,
        start_date: start_date.ok_or(Error::Other("Start Date is required"))?,
        end_data: end_date,
    })
}
