use crate::Context;
use poise::serenity_prelude::{ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, ModalInteraction};
use sea_orm::{DbErr, SqlErr};
use crate::Error;

pub fn is_unique_violation(db_err: &DbErr) -> bool {
    if let Some(SqlErr::UniqueConstraintViolation(_)) = db_err.sql_err() {
        return true;
    }
    false
}

pub fn get_discord_guild_id_from_context(ctx: &Context) -> Option<String> {
    ctx.guild_id().map(|guild_id| guild_id.get().to_string())
}

pub fn get_discord_guild_id_from_interaction(interaction: &ComponentInteraction) -> Option<String> {
    interaction.guild_id.map(|id| id.to_string())
}

pub async fn send_modal_error_response(
    ctx: &Context<'_>,
    interaction: &ModalInteraction,
    error_message: &str,
) -> Result<(), Error> {
    // For modal interactions that haven't been responded to yet
    match interaction
        .create_response(
            ctx.serenity_context(),
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(error_message)
                    .components(vec![])
            )
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::from(err)),
    }
}


pub async fn send_component_followup_error_response(
    ctx: &Context<'_>,
    interaction: &ComponentInteraction,
    error_message: &str,
) -> Result<(), Error> {
    match interaction
        .create_followup(
            &ctx.serenity_context().http,
            CreateInteractionResponseFollowup::new()
                .content(error_message)
                .components(vec![]),
        )
        .await {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::from(err)),
    }
}

pub async fn send_awknowledgement_response(
    ctx: &Context<'_>,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    match interaction
        .create_response(ctx.serenity_context(), CreateInteractionResponse::Acknowledge).await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::from(err)),
    }
}
