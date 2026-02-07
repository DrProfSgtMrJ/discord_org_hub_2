use poise::serenity_prelude::{
    CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal, InputTextStyle,
};
use service::{OrgService, SeasonService};

use super::common::check_dates;
use super::components::{InputText, Modal};
use crate::{Context, Error};

#[poise::command(track_edits, owners_only, slash_command)]
pub async fn create_season(ctx: Context<'_>) -> Result<(), Error> {
    setup_create_season_modal(ctx).await
}

async fn _handle_create_season(
    ctx: Context<'_>,
    title: String,
    num_players: i32,
    start_date: String,
    end_date: Option<String>,
) -> Result<(), Error> {
    let db_service = ctx.data();
    if num_players < 0 {
        ctx.reply("num_players must be non-negative").await?;
    }
    if let Ok((start_date_parsed, end_date_parsed)) = check_dates(start_date, end_date) {
        if let Some(org_id) = ctx.guild_id() {
            let org_discord_id = org_id.get().to_string();
            if let Some(org) = db_service
                .get_org_by_discord_id(org_discord_id.as_str())
                .await?
            {
                let org_id = org.id;
                match db_service
                    .create_season(
                        title.as_str(),
                        org_id,
                        num_players,
                        start_date_parsed,
                        end_date_parsed,
                    )
                    .await
                {
                    Ok(season) => {
                        ctx.reply(format!("Season {} created successfully!", season.id))
                            .await?;
                    }
                    Err(err) => {
                        ctx.reply(format!("Failed to create season: {}", err))
                            .await?;
                    }
                }
            }
        }
    } else {
        ctx.reply("Invalid Date. Please give yyyy-mm-dd").await?;
    }
    Ok(())
}

async fn setup_create_season_modal(ctx: Context<'_>) -> Result<(), Error> {
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
