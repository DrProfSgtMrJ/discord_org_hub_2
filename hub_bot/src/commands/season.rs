use crate::commands::common::get_discord_build_id_from_context;
use poise::CreateReply;
use poise::serenity_prelude::Color;
use poise::serenity_prelude::CreateActionRow;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateInteractionResponse;
use poise::serenity_prelude::CreateInteractionResponseMessage;
use poise::serenity_prelude::CreateModal;
use poise::serenity_prelude::CreateSelectMenu;
use service::{OrderBy, OrgService, SeasonService};

use super::components::{InputText, Modal, SelectMenu};
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

#[poise::command(track_edits, slash_command)]
pub async fn seasons(ctx: Context<'_>) -> Result<(), Error> {
    let db_service = ctx.data();
    if let Some(guild_id) = get_discord_build_id_from_context(&ctx) {
        if let Some(org) = db_service.get_org_by_discord_id(guild_id.as_str()).await? {
            let org_id = org.id;
            let order_by = OrderBy::asc(entity::season::Column::StartDate);
            let seasons = db_service
                .get_seasons_by_org_id(org_id, Some(order_by))
                .await?;

            let mut season_embeded = CreateEmbed::default();
            let mut description = String::new();
            description.push_str(&format!(
                "| {:^2} | {:^30} | {:^10} | {:^10} | {:^7} |\n",
                "ID", "Title", "Start Date", "End Date", "Players"
            ));
            description.push_str("------------------------------------------------\n");

            for (i, season) in seasons.iter().enumerate() {
                description.push_str(
                    format!(
                        "| {:^2} | {:^30} | {:^10} | {:^10} | {:^7} |\n",
                        i + 1,
                        season.title,
                        season.start_date,
                        season
                            .end_date
                            .map_or("-".to_string(), |date| date.to_string()),
                        season.num_players
                    )
                    .as_str(),
                );
                description.push_str("------------------------------------------------\n");
            }
            season_embeded = season_embeded.description(description);
            ctx.send(CreateReply::default().embed(season_embeded))
                .await?;
        }
    }
    //let seasons = db_service.get_seasons_by_org_id(org_id)
    Ok(())
}
