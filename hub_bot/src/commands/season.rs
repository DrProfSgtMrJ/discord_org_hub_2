use poise::serenity_prelude::{CreateActionRow, CreateInteractionResponse, CreateModal};

use super::components::{InputText, Modal};
use crate::{Context, Error};

#[poise::command(track_edits, owners_only, slash_command)]
pub async fn create_season(ctx: Context<'_>) -> Result<(), Error> {
    setup_create_season_modal(ctx).await
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
