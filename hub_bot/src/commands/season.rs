use poise::futures_util::StreamExt;
use poise::serenity_prelude::CreateInteractionResponseMessage;
use std::str::FromStr;
use std::time::Duration;

use crate::commands::common::get_discord_guild_id_from_context;
use poise::CreateReply;
use poise::serenity_prelude::ComponentInteractionCollector;
use poise::serenity_prelude::CreateActionRow;
use poise::serenity_prelude::CreateButton;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateInteractionResponse;
use poise::serenity_prelude::CreateModal;
use service::MemberService;
use service::{OrderBy, OrgService, SeasonService};

use super::components::{Button, InputText, Modal};
use crate::{Context, Error};

const SEASONS_PER_PAGE: usize = 10;

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
    let season_type_action_row: CreateActionRow = InputText::SeasonType.into();

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
                    season_type_action_row,
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
    if let Some(guild_id) = get_discord_guild_id_from_context(&ctx)
        && let Some(org) = db_service.get_org_by_discord_id(&guild_id).await?
    {
        let org_id = org.id;
        let order_by = OrderBy::Asc {
            column: entity::season::Column::StartDate,
        };
        let seasons = db_service
            .get_seasons_by_org_id(org_id, Some(order_by))
            .await?;

        let total_pages = seasons.len().div_ceil(SEASONS_PER_PAGE);
        let mut page = 0usize;

        let build_embed = |page: usize| {
            let start = page * SEASONS_PER_PAGE;
            let end = (start + SEASONS_PER_PAGE).min(seasons.len());

            seasons[start..end].iter().enumerate().fold(
                CreateEmbed::default().title(format!("Seasons ({}/{})", page + 1, total_pages)),
                |embed, (i, season)| {
                    let date_range = match season.end_date {
                        Some(end) => format!("{} - {}", season.start_date, end),
                        None => format!("{} - Present", season.start_date),
                    };
                    embed.field(
                        format!("{}. {}", start + i + 1, season.title),
                        format!("📅 {}", date_range),
                        false,
                    )
                },
            )
        };

        let build_buttons = |page: usize| {
            let prev_disabled = page == 0;
            let next_disabled = page == total_pages - 1;

            let mut prev_button: CreateButton = Button::SeasonsPrev.into();
            let mut next_button: CreateButton = Button::SeasonsNext.into();

            prev_button = prev_button.disabled(prev_disabled);
            next_button = next_button.disabled(next_disabled);

            CreateActionRow::Buttons(vec![prev_button, next_button])
        };

        let components = if total_pages > 1 {
            vec![build_buttons(page)]
        } else {
            vec![]
        };

        let reply = ctx
            .send(
                CreateReply::default()
                    .embed(build_embed(page))
                    .components(components),
            )
            .await?;

        if total_pages <= 1 {
            return Ok(());
        }

        let message = reply.message().await?;

        let mut stream = ComponentInteractionCollector::new(ctx.serenity_context())
            .message_id(message.id)
            .author_id(ctx.author().id)
            .timeout(Duration::from_secs(120))
            .stream();

        while let Some(press) = stream.next().await {
            match Button::from_str(&press.data.custom_id) {
                Ok(Button::SeasonsNext) => {
                    page = (page + 1).min(total_pages - 1);
                }
                Ok(Button::SeasonsPrev) => {
                    page = page.saturating_sub(1);
                }
                _ => continue,
            }

            press
                .create_response(
                    &ctx.serenity_context().http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(build_embed(page))
                            .components(vec![build_buttons(page)]),
                    ),
                )
                .await?;
        }
    }
    Ok(())
}

/// Get Season Info (either current or latest)
///
/// Enter !season_info <season_id>
#[poise::command(track_edits, slash_command)]
pub async fn season_info(
    ctx: Context<'_>,
    #[description = "Season ID as found in /seasons"] season_id: Option<usize>,
) -> Result<(), Error> {
    let db_service = ctx.data();
    if let Some(discord_guild_id) = get_discord_guild_id_from_context(&ctx)
        && let Some(org) = db_service.get_org_by_discord_id(&discord_guild_id).await?
    {
        let org_id = org.id;
        let mut description = String::new();
        let (season_uuid, num_players) = match season_id {
            Some(id) => {
                let order_by = OrderBy::Asc {
                    column: entity::season::Column::StartDate,
                };
                let seasons = db_service
                    .get_seasons_by_org_id(org_id, Some(order_by))
                    .await?;
                if let Some(selected_season) = seasons.get(id - 1) {
                    description.push_str(&format_season_description(id, selected_season));
                    (Some(selected_season.id), selected_season.num_players)
                } else {
                    ctx.reply("Invalid season ID").await?;
                    (None, 0)
                }
            }
            None => {
                if let Some(current_season_uuid) = org.current_season_id
                    && let Some(current_season) =
                        db_service.get_season_by_uuid(current_season_uuid).await?
                {
                    description.push_str(&format_season_description(0, &current_season));
                    (Some(current_season.id), current_season.num_players)
                } else if let Some(latest_season) =
                    db_service.get_latest_season_by_org_id(org_id).await?
                {
                    description.push_str(&format_season_description(0, &latest_season));
                    (Some(latest_season.id), latest_season.num_players)
                } else {
                    ctx.reply("Unable to get season info").await?;
                    (None, 0)
                }
            }
        };
        description.push_str("---------------------------------------\n");
        if let Some(season_uuid) = season_uuid {
            let order_by = OrderBy::AscNullsFirst {
                column: entity::season_member::Column::Placement,
            };
            let members = db_service
                .get_members_in_season(season_uuid, Some(order_by))
                .await?;
            for member in members {
                println!("Got member {:?}", member.display_name);
                description.push_str(&format!(
                    "**{}**: \t{}/{}\n",
                    member.display_name.unwrap_or_default(),
                    member
                        .season_member
                        .placement
                        .map_or("-".to_string(), |p| p.to_string()),
                    num_players,
                ));
            }
        }
        let season_embeded = CreateEmbed::default().description(description);
        ctx.send(CreateReply::default().embed(season_embeded))
            .await?;
    }

    Ok(())
}

fn format_season_description(season_id: usize, season: &entity::season::Model) -> String {
    let date_range = match season.end_date {
        Some(end) => format!("{} - {}", season.start_date, end),
        None => format!("{} - Present", season.start_date),
    };
    format!(
        "**{}** • {} \n📅 {}\n\n",
        season_id, season.title, date_range
    )
}
