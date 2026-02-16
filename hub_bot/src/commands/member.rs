use super::common::is_unique_violation;
use poise::serenity_prelude::Member;
use service::MemberService;
use service::OrgService;
use service::UserService;

use crate::commands::common::get_discord_build_id_from_context;
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
    member_6: Option<Member>,
    member_7: Option<Member>,
    member_8: Option<Member>,
    member_9: Option<Member>,
    member_10: Option<Member>,
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
    if let Some(member_6) = member_6 {
        members.push(member_6);
    }
    if let Some(member_7) = member_7 {
        members.push(member_7);
    }
    if let Some(member_8) = member_8 {
        members.push(member_8);
    }
    if let Some(member_9) = member_9 {
        members.push(member_9);
    }
    if let Some(member_10) = member_10 {
        members.push(member_10);
    }

    for member in members {
        handle_register_member(ctx.clone(), member).await?;
    }

    Ok(())
}

pub async fn handle_register_member(ctx: Context<'_>, member: Member) -> Result<(), Error> {
    if let Some(discord_guild_id) = get_discord_build_id_from_context(&ctx) {
        let db_service = ctx.data();
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

    Ok(())
}
