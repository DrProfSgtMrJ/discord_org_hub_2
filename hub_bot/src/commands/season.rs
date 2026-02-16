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
    if let Some(guild_id) = get_discord_build_id_from_context(&ctx)
        && let Some(org) = db_service.get_org_by_discord_id(&guild_id).await?
    {
        let org_id = org.id;
        let order_by = OrderBy::asc(entity::season::Column::StartDate);
        let seasons = db_service
            .get_seasons_by_org_id(org_id, Some(order_by))
            .await?;

        let mut season_embeded = CreateEmbed::default();
        let mut description = String::new();

        for (i, season) in seasons.iter().enumerate() {
            let date_range = match season.end_date {
                Some(end) => format!("{} - {}", season.start_date, end),
                None => format!("{} - Present", season.start_date),
            };

            description.push_str(
                format!("**{}** • {} \n📅 {}\n\n", i + 1, season.title, date_range,).as_str(),
            );
        }
        season_embeded = season_embeded.description(description);
        ctx.send(CreateReply::default().embed(season_embeded))
            .await?;
    }
    Ok(())
}

/// Get Season Info
///
/// Enter !season_info <season_id>
#[poise::command(track_edits, slash_command)]
pub async fn season_info(ctx: Context<'_>, season_id: Option<u64>) -> Result<(), Error> {
    todo!()
}
