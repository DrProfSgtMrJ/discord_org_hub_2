use poise::serenity_prelude::{
    ComponentInteraction, CreateActionRow, CreateButton, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateModal,
    ModalInteraction,
};

use super::data::SeasonParsedData;
use crate::common::send_component_followup_error_response;
use crate::components::{Button, InputText, Modal, SelectMenu};
use crate::{Context, Error};
use entity::sea_orm_active_enums::SeasonType;

use service::{OrgService, SeasonService};

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

pub async fn send_season_type_select_menu(
    ctx: &Context<'_>,
    interaction: &ModalInteraction,
) -> Result<(), Error> {
    let select_season_type_menu: CreateActionRow = SelectMenu::SeasonTypeSelectMenu.into();
    interaction
        .create_response(
            &ctx.serenity_context().http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Select the season type")
                    .components(vec![select_season_type_menu]),
            ),
        )
        .await?;
    Ok(())
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
                        "Failed to create season",
                    )
                    .await?;
                }
            }
        } else {
            send_component_followup_error_response(
                ctx,
                component_interaction,
                "Organization not found",
            )
            .await?;
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

    let skip_button: CreateButton = Button::MemberSelectSkip.into();
    let skip_button_row = CreateActionRow::Buttons(vec![skip_button]);

    component_interaction
        .create_followup(
            &ctx.serenity_context().http,
            CreateInteractionResponseFollowup::new()
                .content(format!(
                    "Season: '{}' created successfully. Add members to season?",
                    season_title
                ))
                .components(vec![select_user_menu, skip_button_row]),
        )
        .await?;

    Ok(())
}

pub async fn send_set_season_as_current(
    season_uuid: &str,
    season_title: &str,
    ctx: &Context<'_>,
    component_interaction: &ComponentInteraction,
) -> Result<(), Error> {
    let set_current_yes_button: CreateButton = Button::SetAsCurrentSeasonYes {
        season_uuid: season_uuid.to_string(),
    }
    .into();
    let set_current_no_button: CreateButton = Button::SetAsCurrentSeasonNo {
        season_uuid: season_uuid.to_string(),
    }
    .into();

    let button_action_row: CreateActionRow =
        CreateActionRow::Buttons(vec![set_current_yes_button, set_current_no_button]);

    component_interaction
        .create_followup(
            &ctx.serenity_context().http,
            CreateInteractionResponseFollowup::new()
                .content(format!("Set {} as current season?", season_title))
                .components(vec![button_action_row]),
        )
        .await?;

    Ok(())
}
