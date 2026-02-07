use poise::serenity_prelude::Member;
use service::OrgService;
use service::UserService;

use crate::commands::common::is_unique_violation;
use crate::{Context, Error};

/// Command to register an org
///
/// Enter !register_org @<owner> <org_name>
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn register_org(ctx: Context<'_>, member: Member, org_name: String) -> Result<(), Error> {
    //Adds a new Org to the database
    if let Some(org_id) = ctx.guild_id() {
        let org_discord_id = org_id.get().to_string();
        let owner_id = member.user.id.get().to_string();
        let db_service = ctx.data();
        if let Some(owner) = db_service.get_user_by_discord_id(owner_id.as_str()).await? {
            match db_service
                .create_org(org_name.as_str(), org_discord_id.as_str(), owner.id, None)
                .await
            {
                Ok(org) => {
                    ctx.reply(format!(
                        "Organization {} registered successfully!",
                        org.name
                    ))
                    .await?;
                }
                Err(err) => {
                    // Handle the error
                    if is_unique_violation(&err) {
                        ctx.reply(format!(
                            "Organization with name {} already exists.",
                            org_name
                        ))
                        .await?;
                    } else {
                        ctx.reply(format!(
                            "An error occurred while registering the organization: {}",
                            err
                        ))
                        .await?;
                    }
                }
            }
        } else {
            ctx.reply(format!(
                "User with Discord ID {} not found. Please register as a user first.",
                owner_id
            ))
            .await?;
        }
    }
    Ok(())
}
