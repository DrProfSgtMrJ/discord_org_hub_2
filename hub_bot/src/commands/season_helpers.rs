use std::str::FromStr;
use poise::serenity_prelude::{ComponentInteraction, ComponentInteractionCollector, ComponentInteractionDataKind, CreateActionRow, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateModal, ModalInteraction, ModalInteractionCollector};
use std::sync::Arc;
use std::time::Duration;

use crate::commands::common::{get_discord_guild_id_from_interaction, send_awknowledgement_response, send_component_followup_error_response};
use crate::components::{Modal, InputText, SelectMenu};
use crate::{Context, Error};
use super::common::send_modal_error_response;
use entity::sea_orm_active_enums::SeasonType;
use super::data::{SeasonParsedData, SeasonFormData};

use service::{OrgService, SeasonService, MemberService};

pub async fn setup_create_season_modal(ctx: &Context<'_>) -> Result<(), Error> {
    let modal: CreateModal = Modal::CreateSeason.into();
    let title_action_row: CreateActionRow = InputText::SeasonTitle.into();
    let num_players_action_row: CreateActionRow = InputText::SeasonNumPlayers.into();
    let start_date_action_row: CreateActionRow = InputText::SeasonStartDate.into();
    let end_date_action_row: CreateActionRow = InputText::SeasonEndDate.into();

    if let poise::Context::Application(app_ctx) = ctx {
        app_ctx
            .interaction
            .create_response(
                &ctx.serenity_context().http,
                CreateInteractionResponse::Modal(modal.components(vec![
                    title_action_row,
                    num_players_action_row,
                    start_date_action_row,
                    end_date_action_row,
                ])),
            )
            .await?;
    } else {
        ctx.reply("This command only works as a slash command. Please use /create_server")
            .await?;
    }


    Ok(())
}


pub async fn handle_modal_submission(ctx: &Context<'_>) -> Result<(SeasonParsedData, Arc<ModalInteraction>), Error> {
    let Some(modal_response) = ModalInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .custom_ids(vec![Modal::CreateSeason.id()])
        .timeout(Duration::from_secs(300))
        .await else {
            return Err("Failed to collect modal response".into());
        };

    match SeasonFormData::from_action_rows(&modal_response.data.components)
        .and_then(|f| f.parsed_data())
    {
        Ok(parsed_data) => Ok((parsed_data, Arc::new(modal_response))),
        Err(_) => {
            send_modal_error_response(
                ctx,
                &modal_response,
                "Failed to parse form data",
            )
            .await?;
            Err("Failed to parse form data".into())
        }
    }
}

pub async fn send_season_type_select_menu(
    ctx: &Context<'_>,
    interaction: &ModalInteraction,
) -> Result<(), Error> {
    let select_season_type_menu: CreateActionRow = SelectMenu::SeasonTypeSelectMenu.into();
    interaction
        .create_response(
            &ctx.serenity_context().http,
            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                .content("Select the season type")
                .components(vec![select_season_type_menu]),
            ),
        )
        .await?;
    Ok(())
}

pub async fn handle_season_type_select_menu(
    ctx: &Context<'_>
) -> Result<(SeasonType, Arc<ComponentInteraction>), Error> {
    let Some(component_response) = ComponentInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .custom_ids(vec![SelectMenu::SeasonTypeSelectMenu.id()])
        .timeout(Duration::from_secs(300))
        .await else {
            return Err("Failed to collect component response".into());
        };
    let selected_value = match &component_response.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => values.first().ok_or("No season type selected")?,
        _ => return Err("Unexpected component interaction type".into()),
    };

    let season_type = SeasonType::from_str(selected_value).map_err(|_| format!("Invalid season type: {}", selected_value))?;
    
    send_awknowledgement_response(ctx, &component_response).await?;
    
    Ok((season_type, Arc::new(component_response)))
}

pub async fn create_season_with_type(
    ctx: &Context<'_>,
    component_interaction: &ComponentInteraction,
    season_data: SeasonParsedData,
    season_type: SeasonType,
) -> Result<entity::season::Model, Error> {
    if let Some(org_id) = component_interaction.guild_id {
        let org_discord_id = org_id.get().to_string();
        let db_service = &ctx.data().db_service;
        if let Some(org) = db_service
            .get_org_by_discord_id(org_discord_id.as_str())
            .await?
        {
            let org_id = org.id;
            match db_service
                .create_season(
                    season_data.title.as_str(),
                    org_id,
                    season_data.num_players,
                    season_data.start_date,
                    season_data.end_date,
                    Some(season_type),
                )
                .await
            {
                Ok(season) => {
                    return Ok(season);
                }
                Err(_) => {
                    send_component_followup_error_response(
                        ctx, 
                        component_interaction,
                        "Failed to create season"
                    ).await?;
                }
            }
        } else {
            send_component_followup_error_response(
                ctx, 
                component_interaction, 
                "Organization not found"
            ).await?;
        }
    }
    Err("Failed to create season".into())
}

pub async fn send_add_members_to_season(
    season_uuid: &str,
    season_title: &str,
    ctx: &Context<'_>,
    component_interaction: &ComponentInteraction,
) -> Result<(), Error> {
    let select_user_menu: CreateActionRow = SelectMenu::MemberSelectMenu {
        season_uuid: season_uuid.to_string(),
    }
    .into();
    component_interaction
        .create_followup(
            &ctx.serenity_context().http,
            CreateInteractionResponseFollowup::new()
                .content(format!(
                    "Season: '{}' created successfully. Add members to season?",
                    season_title
                ))
                .components(vec![select_user_menu]),
        )
        .await?;

    Ok(())
}

pub async fn handle_add_memebers_to_season(ctx: &Context<'_>, season_uuid: &str) -> Result<Arc<ComponentInteraction>, Error> {
    let Some(component_response) = ComponentInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .custom_ids(vec![SelectMenu::MemberSelectMenu { season_uuid: season_uuid.to_string() }.id()])
        .timeout(Duration::from_secs(300))
        .await else {
            return Err("Failed to collect component response".into());
        };

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
            match ctx.data().db_service.get_member_by_ids(&discord_user_id, &discord_guild_id).await {
                Ok(Some(member)) => {
                    match ctx.data().db_service.add_member_to_season(member.id, season_uuid, None).await {
                        Ok(_) => {
                            success_messages.push(format!("Added {}", &discord_user_id));
                        }
                        Err(_) => {
                            error_messages.push(format!("Unable to add user with ID {}", discord_user_id));
                        }
                    }
                }
                Ok(None) => {
                    error_messages.push(format!("Failed to get member with discord user_id: {}", discord_user_id));
                }
                Err(_) => {
                    error_messages.push(format!("Database error while fetching member with discord user_id: {}", discord_user_id));
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

    component_response.create_response(
        ctx.serenity_context(),
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(response_content)
                .components(vec![])
        )
    ).await?;

    Ok(Arc::new(component_response))
}

