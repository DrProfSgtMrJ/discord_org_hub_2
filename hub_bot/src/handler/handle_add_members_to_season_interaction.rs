use poise::serenity_prelude::{
    ComponentInteraction, ComponentInteractionDataKind, Context, CreateActionRow, CreateButton,
    CreateInteractionResponseFollowup,
};
use service::{DbService, MemberService};
use uuid::Uuid;

use crate::Error;
use crate::commands::Button;
use crate::handler::common::{
    get_discord_guild_id_from_interaction, send_component_error_response, send_success_response,
};

pub async fn handle_add_members_to_season_interaction(
    db_service: &DbService,
    season_uuid: &str,
    ctx: &Context,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    interaction.defer(&ctx.http).await?;
    let user_ids = match &interaction.data.kind {
        ComponentInteractionDataKind::UserSelect { values } => values.clone(),
        _ => {
            send_component_error_response(ctx, interaction, "Unexpected interaction data kind")
                .await?;
            return Ok(());
        }
    };
    let season_uuid = Uuid::parse_str(season_uuid)?;
    if let Some(discord_guild_id) = get_discord_guild_id_from_interaction(interaction) {
        for user_id in user_ids {
            let discord_user_id = user_id.get().to_string();
            match db_service
                .get_member_by_ids(&discord_user_id, &discord_guild_id)
                .await
            {
                Ok(Some(member)) => {
                    match db_service
                        .add_member_to_season(member.id, season_uuid, None)
                        .await
                    {
                        Ok(_) => {
                            send_success_response(ctx, interaction).await?;
                        }
                        Err(_) => {
                            send_component_error_response(
                                ctx,
                                interaction,
                                &format!("Unable to add user with ID {}", discord_guild_id),
                            )
                            .await?;
                        }
                    }
                }
                Ok(None) => {
                    send_component_error_response(
                        ctx,
                        interaction,
                        &format!(
                            "Failed to get member with discord user_id: {}",
                            discord_user_id
                        ),
                    )
                    .await?;
                }
                Err(_) => {
                    send_component_error_response(
                        ctx,
                        interaction,
                        &format!(
                            "Failed to get member with discord user_id: {}",
                            discord_user_id
                        ),
                    )
                    .await?;
                }
            }
        }

        send_set_season_as_current_button(&season_uuid.to_string(), ctx, interaction).await?;
    }

    Ok(())
}

async fn send_set_season_as_current_button(
    season_uuid: &str,
    ctx: &Context,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    let yes_button = Button::SetAsCurrentSeasonYes {
        season_uuid: season_uuid.to_string(),
    };
    let no_button = Button::SetAsCurrentSeasonNo {
        season_uuid: season_uuid.to_string(),
    };
    let create_yes_button: CreateButton = yes_button.into();
    let create_no_button: CreateButton = no_button.into();
    let button_row = CreateActionRow::Buttons(vec![create_yes_button, create_no_button]);
    interaction
        .create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new()
                .content("Set as current season?")
                .components(vec![button_row]),
        )
        .await?;
    Ok(())
}
