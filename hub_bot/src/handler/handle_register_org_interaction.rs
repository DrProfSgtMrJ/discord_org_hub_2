use super::common::{send_component_error_response, send_success_response};
use poise::serenity_prelude::{ComponentInteraction, Context};
use service::{DbService, OrgService, UserService};

use crate::Error;

pub async fn handle_register_org_interaction_yes(
    db_service: &DbService,
    ctx: &Context,
    interaction: &ComponentInteraction,
    org_name: &str,
    guild_id: &str,
    owner_id: &str,
) -> Result<(), Error> {
    if let Some(owner) = db_service.get_user_by_discord_id(owner_id).await? {
        match db_service
            .create_org(org_name, guild_id, owner.id, None)
            .await
        {
            Ok(_) => {
                send_success_response(ctx, interaction).await?;
            }
            Err(_) => {
                send_component_error_response(ctx, interaction, "Failed to register org").await?;
            }
        }
    } else {
        send_component_error_response(
            ctx,
            interaction,
            "Please register your account first (do `/join`)",
        )
        .await?;
    }
    Ok(())
}
