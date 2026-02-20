use super::common::is_unique_violation;
use poise::serenity_prelude::Member;
use service::MemberService;
use service::OrderBy;
use service::OrgService;
use service::SeasonService;
use service::UserService;

use crate::commands::common::get_discord_guild_id_from_context;
use crate::{Context, Error};

/// Command to register a member to an organization
///
/// Enter !register_member @member to register the mentioned member to the organization associated with the current guild
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn register_member(
    ctx: Context<'_>,
    #[description = "The member to register"] member: Member,
) -> Result<(), Error> {
    handle_register_member(ctx, member).await
}

/// Command to register members to an organization
///
/// Enter !register_members @member, @member ... to register the mentioned member(s) to the organization associated with the current guild
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn register_members(
    ctx: Context<'_>,
    #[description = "The member to register"] member: Member,
    member_1: Option<Member>,
    member_2: Option<Member>,
    member_3: Option<Member>,
    member_4: Option<Member>,
    member_5: Option<Member>,
) -> Result<(), Error> {
    let mut members = vec![member];
    if let Some(member_1) = member_1 {
        members.push(member_1);
    }
    if let Some(member_2) = member_2 {
        members.push(member_2);
    }
    if let Some(member_3) = member_3 {
        members.push(member_3);
    }
    if let Some(member_4) = member_4 {
        members.push(member_4);
    }
    if let Some(member_5) = member_5 {
        members.push(member_5);
    }

    for member in members {
        handle_register_member(ctx, member).await?;
    }

    Ok(())
}

pub async fn handle_register_member(ctx: Context<'_>, member: Member) -> Result<(), Error> {
    if let Some(discord_guild_id) = get_discord_guild_id_from_context(&ctx) {
        let db_service = &ctx.data().db_service;
        let discord_user_id = member.user.id.get().to_string();
        if let Some(org) = db_service.get_org_by_discord_id(&discord_guild_id).await?
            && let Some(user) = db_service.get_user_by_discord_id(&discord_user_id).await?
        {
            let org_uuid = org.id;
            let user_uuid = user.id;
            match db_service.create_member(user_uuid, org_uuid).await {
                Ok(_) => {
                    ctx.reply(format!(
                        "Member {} registered successfully!",
                        user.display_name
                    ))
                    .await?;
                }
                Err(err) => {
                    if is_unique_violation(&err) {
                        ctx.reply(format!(
                            "Member with Discord ID {} already exists in the organization",
                            discord_user_id
                        ))
                        .await?;
                    } else {
                        ctx.reply("An unexpected database error occurred").await?;
                    }
                }
            }
        } else {
            ctx.reply(&format!(
                "Please check if Org {} or User {} is registered",
                discord_guild_id, discord_user_id
            ))
            .await?;
        }
    }

    ctx.defer().await?;
    Ok(())
}

/// Command to add a member to a season
///
/// Enter !add_to_season <season number> @member to add member to the season
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn add_to_season(
    ctx: Context<'_>,
    #[description = "Season ID as found in /seasons"] season_id: usize,
    member: Member,
    #[description = "Placement in the season"] placement: Option<i32>,
) -> Result<(), Error> {
    if let Some(discord_guild_id) = get_discord_guild_id_from_context(&ctx) {
        let db_service = &ctx.data().db_service;
        let discord_user_id = member.user.id.get().to_string();
        if let Some(member) = db_service
            .get_member_by_ids(&discord_user_id, &discord_guild_id)
            .await?
        {
            let org_id = member.org_id;
            let order_by = OrderBy::Asc {
                column: entity::season::Column::StartDate,
            };
            let seasons = db_service
                .get_seasons_by_org_id(org_id, Some(order_by))
                .await?;
            if let Some(selected_season) = seasons.get(season_id - 1) {
                match db_service
                    .add_member_to_season(member.id, selected_season.id, placement)
                    .await
                {
                    Ok(_) => {
                        ctx.reply("Member added successfully").await?;
                    }
                    Err(err) => {
                        if is_unique_violation(&err) {
                            ctx.reply("Member already exists in season").await?;
                        } else {
                            ctx.reply("Failed to add member to season").await?;
                        }
                    }
                }
            } else {
                ctx.reply("Invalid season number").await?;
            }
        }
    }
    Ok(())
}
