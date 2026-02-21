use entity::sea_orm_active_enums::SeasonType;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::CreateInteractionResponseMessage;
use sea_orm::ActiveEnum;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use crate::commands::common::{get_discord_guild_id_from_context, send_success_response};
use poise::CreateReply;
use poise::serenity_prelude::{
    ComponentInteractionCollector, CreateActionRow, CreateButton, CreateEmbed,
    CreateInteractionResponse,
};
use service::MemberService;
use service::{OrderBy, OrgService, SeasonService};

use super::season_helpers::{
    create_season_with_type, handle_add_memebers_to_season, handle_modal_submission,
    handle_season_type_select_menu, handle_set_current_season, send_add_members_to_season,
    send_season_type_select_menu, send_set_season_as_current, setup_create_season_modal,
};
use crate::components::Button;
use crate::{Context, Error};

const SEASONS_PER_PAGE: usize = 10;

#[poise::command(track_edits, owners_only, slash_command)]
pub async fn create_season(ctx: Context<'_>) -> Result<(), Error> {
    // Open Modal
    setup_create_season_modal(&ctx).await?;
    let (parsed_season_data, modal_response) = handle_modal_submission(&ctx).await?;
    send_season_type_select_menu(&ctx, &modal_response).await?;
    let (season_type, component_interaction) = handle_season_type_select_menu(&ctx).await?;
    let season = create_season_with_type(
        &ctx,
        &component_interaction,
        parsed_season_data,
        season_type,
    )
    .await?;
    send_add_members_to_season(
        &season.id.to_string(),
        &season.title,
        &ctx,
        &component_interaction,
    )
    .await?;
    let member_component_interaction =
        handle_add_memebers_to_season(&ctx, &season.id.to_string()).await?;
    send_set_season_as_current(
        &season.id.to_string(),
        &season.title,
        &ctx,
        &member_component_interaction,
    )
    .await?;
    handle_set_current_season(&ctx, &season, &member_component_interaction).await?;
    send_success_response(&ctx, &member_component_interaction).await?;
    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn seasons(ctx: Context<'_>) -> Result<(), Error> {
    let db_service = &ctx.data().db_service;
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

        let mut grouped: HashMap<SeasonType, Vec<entity::season::Model>> = HashMap::new();
        for season in &seasons {
            grouped
                .entry(season.season_type.clone())
                .or_default()
                .push(season.clone())
        }

        let mut page = 0usize;
        let mut active_filter: Option<SeasonType> = None;

        let get_view = |filter: &Option<SeasonType>| -> &[entity::season::Model] {
            match filter {
                Some(ft) => grouped.get(ft).map(|v| v.as_slice()).unwrap_or(&[]),
                None => seasons.as_slice(),
            }
        };

        let view = get_view(&active_filter);
        let total_pages = view.len().div_ceil(SEASONS_PER_PAGE).max(1);

        let reply = ctx
            .send(
                CreateReply::default()
                    .embed(build_embed(view, &active_filter, page, total_pages))
                    .components(vec![
                        build_nav_row(page, total_pages),
                        build_filter_row(&active_filter),
                    ]),
            )
            .await?;

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
                Ok(Button::SeasonTheChallenge { .. }) => {
                    active_filter = toggle_filter(active_filter, SeasonType::TheChallenge);
                    page = 0;
                }
                Ok(Button::SeasonSurvivor { .. }) => {
                    active_filter = toggle_filter(active_filter, SeasonType::Survivor);
                    page = 0;
                }
                Ok(Button::SeasonBigBrother { .. }) => {
                    active_filter = toggle_filter(active_filter, SeasonType::BigBrother);
                    page = 0;
                }
                Ok(Button::SeasonOther { .. }) => {
                    active_filter = toggle_filter(active_filter, SeasonType::Other);
                    page = 0;
                }
                Ok(Button::SeasonTraitors { .. }) => {
                    active_filter = toggle_filter(active_filter, SeasonType::Traitors);
                    page = 0;
                }
                _ => continue,
            }

            // recomputing to avoid stale data
            let view = get_view(&active_filter);
            let total_pages = view.len().div_ceil(SEASONS_PER_PAGE).max(1);
            page = page.min(total_pages.saturating_sub(1));

            press
                .create_response(
                    &ctx.serenity_context().http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(build_embed(view, &active_filter, page, total_pages))
                            .components(vec![
                                build_nav_row(page, total_pages),
                                build_filter_row(&active_filter),
                            ]),
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
    let db_service = &ctx.data().db_service;
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

fn build_embed(
    seasons: &[entity::season::Model],
    active_filter: &Option<SeasonType>,
    page: usize,
    total_pages: usize,
) -> CreateEmbed {
    let start = page * SEASONS_PER_PAGE;
    let end = (start + SEASONS_PER_PAGE).min(seasons.len());
    let title = match active_filter {
        Some(ft) => format!(
            "Seasons: ({}) ({})/({})",
            ft.to_value(),
            page + 1,
            total_pages
        ),
        None => "Seasons (All)".to_string(),
    };
    seasons[start..end].iter().enumerate().fold(
        CreateEmbed::default().title(title),
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
}

fn build_nav_row(page: usize, total_pages: usize) -> CreateActionRow {
    let prev_disabled = page == 0;
    let next_disabled = page == total_pages - 1;

    let mut prev_button: CreateButton = Button::SeasonsPrev.into();
    let mut next_button: CreateButton = Button::SeasonsNext.into();

    prev_button = prev_button.disabled(prev_disabled);
    next_button = next_button.disabled(next_disabled);

    CreateActionRow::Buttons(vec![prev_button, next_button])
}

fn build_filter_row(active_filter: &Option<SeasonType>) -> CreateActionRow {
    CreateActionRow::Buttons(vec![
        Button::SeasonSurvivor {
            is_active: matches!(active_filter, Some(SeasonType::Survivor)),
        }
        .into(),
        Button::SeasonBigBrother {
            is_active: matches!(active_filter, Some(SeasonType::BigBrother)),
        }
        .into(),
        Button::SeasonTraitors {
            is_active: matches!(active_filter, Some(SeasonType::Traitors)),
        }
        .into(),
        Button::SeasonTheChallenge {
            is_active: matches!(active_filter, Some(SeasonType::TheChallenge)),
        }
        .into(),
        Button::SeasonOther {
            is_active: matches!(active_filter, Some(SeasonType::Other)),
        }
        .into(),
    ])
}

fn toggle_filter(current: Option<SeasonType>, new: SeasonType) -> Option<SeasonType> {
    match current {
        Some(filter) if filter == new => None, // passing an active filter will clear it
        _ => Some(new),
    }
}
