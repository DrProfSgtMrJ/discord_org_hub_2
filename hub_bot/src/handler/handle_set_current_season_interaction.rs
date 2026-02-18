use super::common::send_success_response;
use poise::serenity_prelude::{
    ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use service::{DbService, OrgService, SeasonService};

use crate::Error;

pub async fn handle_set_current_season_interaction_yes(
    db_service: &DbService,
    season_uuid: &str,
    ctx: &Context,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    if let Ok(season) = db_service.get_season_by_id(season_uuid).await {
        match season {
            Some(season) => match db_service
                .set_current_season(season.org_id, season.id)
                .await
            {
                Ok(_) => {
                    send_success_response(ctx, interaction).await?;
                }
                Err(err) => {
                    interaction
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content(format!("Error setting current season: {}", err)),
                            ),
                        )
                        .await?;
                }
            },
            None => send_season_not_found_message(ctx, interaction).await?,
        }
    } else {
        send_season_not_found_message(ctx, interaction).await?
    }

    Ok(())
}

pub async fn handle_set_current_season_interaction_no(
    _db_service: &DbService,
    _season_uuid: &str,
    ctx: &Context,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Success!"),
            ),
        )
        .await?;
    Ok(())
}

async fn send_season_not_found_message(
    ctx: &Context,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Season not found"),
            ),
        )
        .await?;
    Ok(())
}
