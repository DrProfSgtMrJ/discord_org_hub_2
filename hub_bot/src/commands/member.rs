use super::common::is_unique_violation;
use poise::serenity_prelude::Member;
use service::MemberService;
use service::OrgService;
use service::UserService;

use crate::{Context, Error};

/// Command to register a member to an organization
///
/// Enter !register_member @member to register the mentioned member to the organization associated with the current guild
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn register_member(
    ctx: Context<'_>,
    #[description = "The member to register"] member: Member,
) -> Result<(), Error> {
    if let Some(org_id) = ctx.guild_id() {
        let org_discord_id = org_id.get().to_string();
        let discord_user_id = member.user.id.get().to_string();

        let db_service = ctx.data();

        if let Some(org) = db_service.get_org_by_discord_id(&org_discord_id).await? {
            if let Some(user) = db_service.get_user_by_discord_id(&discord_user_id).await? {
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
                ctx.reply(format!(
                    "User with Discord ID {} not found. Please register as a user first.",
                    discord_user_id
                ))
                .await?;
            }
        } else {
            ctx.reply(format!(
                "Organization with Discord ID {} not found. Please register the organization first.",
                org_discord_id
            )).await?;
        }
    }
    Ok(())
}
