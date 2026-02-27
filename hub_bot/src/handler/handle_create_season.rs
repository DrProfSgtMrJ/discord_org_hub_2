use crate::commands::{SeasonFormData, SeasonParsedData};
use crate::common::{
    get_discord_guild_id_from_interaction, send_awknowledgement_response,
    send_component_followup_error_response, send_modal_error_response,
};
use crate::components::{Button, Modal, SelectMenu};
use crate::{Context, Error};
use entity::sea_orm_active_enums::SeasonType;
use service::{MemberService, OrgService};

use poise::serenity_prelude::{
    ComponentInteraction, ComponentInteractionCollector, ComponentInteractionDataKind,
    CreateInteractionResponse, CreateInteractionResponseMessage, ModalInteraction,
    ModalInteractionCollector,
};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

pub async fn handle_create_season_modal_submission(
    ctx: &Context<'_>,
) -> Result<(SeasonParsedData, Arc<ModalInteraction>), Error> {
    let Some(modal_response) = ModalInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .custom_ids(vec![Modal::CreateSeason.id()])
        .timeout(Duration::from_secs(300))
        .await
    else {
        return Err("Failed to collect modal response".into());
    };

    match SeasonFormData::from_action_rows(&modal_response.data.components)
        .and_then(|f| f.parsed_data())
    {
        Ok(parsed_data) => Ok((parsed_data, Arc::new(modal_response))),
        Err(_) => {
            send_modal_error_response(ctx, &modal_response, "Failed to parse form data").await?;
            Err("Failed to parse form data".into())
        }
    }
}

pub async fn handle_season_type_select_menu(
    ctx: &Context<'_>,
) -> Result<(SeasonType, Arc<ComponentInteraction>), Error> {
    let Some(component_response) = ComponentInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .custom_ids(vec![SelectMenu::SeasonTypeSelectMenu.id()])
        .timeout(Duration::from_secs(300))
        .await
    else {
        return Err("Failed to collect component response".into());
    };
    let selected_value = match &component_response.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => {
            values.first().ok_or("No season type selected")?
        }
        _ => return Err("Unexpected component interaction type".into()),
    };

    let season_type = SeasonType::from_str(selected_value)
        .map_err(|_| format!("Invalid season type: {}", selected_value))?;

    send_awknowledgement_response(ctx, &component_response).await?;

    Ok((season_type, Arc::new(component_response)))
}

pub async fn handle_add_memebers_to_season(
    ctx: &Context<'_>,
    season_uuid: &str,
) -> Result<Arc<ComponentInteraction>, Error> {
    let member_select_id = SelectMenu::MemberSelectMenu {
        season_uuid: season_uuid.to_string(),
    }
    .id();
    let member_skip_id = Button::MemberSelectSkip.id();

    let Some(component_response) = ComponentInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .custom_ids(vec![member_select_id, member_skip_id])
        .timeout(Duration::from_secs(300))
        .await
    else {
        return Err("Failed to collect component response".into());
    };

    if matches!(
        Button::from_str(&component_response.data.custom_id),
        Ok(Button::MemberSelectSkip)
    ) {
        send_awknowledgement_response(ctx, &component_response).await?;
        return Ok(Arc::new(component_response));
    }

    let selected_user_ids = match &component_response.data.kind {
        ComponentInteractionDataKind::UserSelect { values } => values.clone(),
        _ => return Err("Unexpected component interaction type".into()),
    };

    let season_uuid = uuid::Uuid::parse_str(season_uuid)?;
    let mut success_messages = Vec::new();
    let mut error_messages = Vec::new();

    if let Some(discord_guild_id) = get_discord_guild_id_from_interaction(&component_response) {
        for user_id in selected_user_ids {
            let discord_user_id = user_id.get().to_string();
            match ctx
                .data()
                .db_service
                .get_member_by_ids(&discord_user_id, &discord_guild_id)
                .await
            {
                Ok(Some(member)) => {
                    match ctx
                        .data()
                        .db_service
                        .add_member_to_season(member.id, season_uuid, None)
                        .await
                    {
                        Ok(_) => {
                            success_messages.push(format!("Added {}", &discord_user_id));
                        }
                        Err(_) => {
                            error_messages
                                .push(format!("Unable to add user with ID {}", discord_user_id));
                        }
                    }
                }
                Ok(None) => {
                    error_messages.push(format!(
                        "Failed to get member with discord user_id: {}",
                        discord_user_id
                    ));
                }
                Err(_) => {
                    error_messages.push(format!(
                        "Database error while fetching member with discord user_id: {}",
                        discord_user_id
                    ));
                }
            }
        }
    }

    // Send a single response with all results
    let mut response_content = String::new();
    if !success_messages.is_empty() {
        response_content.push_str("**Successfully added:**\n");
        for msg in &success_messages {
            response_content.push_str(&format!("✅ {}\n", msg));
        }
    }
    if !error_messages.is_empty() {
        if !success_messages.is_empty() {
            response_content.push('\n');
        }
        response_content.push_str("**Errors:**\n");
        for msg in &error_messages {
            response_content.push_str(&format!("❌ {}\n", msg));
        }
    }

    if response_content.is_empty() {
        response_content = "No users were processed.".to_string();
    }

    component_response
        .create_response(
            ctx.serenity_context(),
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(response_content)
                    .components(vec![]),
            ),
        )
        .await?;

    Ok(Arc::new(component_response))
}

pub async fn handle_set_current_season(
    ctx: &Context<'_>,
    season: &entity::season::Model,
) -> Result<(), Error> {
    let Some(component_response) = ComponentInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .custom_ids(vec![
            Button::SetAsCurrentSeasonNo {
                season_uuid: season.id.to_string(),
            }
            .id(),
            Button::SetAsCurrentSeasonYes {
                season_uuid: season.id.to_string(),
            }
            .id(),
        ])
        .timeout(Duration::from_secs(300))
        .await
    else {
        return Err("Failed to collect component response".into());
    };

    match Button::from_str(&component_response.data.custom_id) {
        Ok(Button::SetAsCurrentSeasonYes { .. }) => {
            ctx.data()
                .db_service
                .set_current_season(season.org_id, season.id)
                .await?;
        }
        Ok(Button::SetAsCurrentSeasonNo { .. }) => {}
        Err(e) => return Err(e.into()),

        _ => {
            send_component_followup_error_response(
                ctx,
                &component_response,
                "Invalid Button Interaction",
            )
            .await?;
        }
    }
    send_awknowledgement_response(ctx, &component_response).await?;
    Ok(())
}
