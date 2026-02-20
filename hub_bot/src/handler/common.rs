use crate::Error;
use poise::serenity_prelude::{
    ComponentInteraction, Context as SerenityContext, CreateInteractionResponseFollowup,
    ModalInteraction,
};

pub fn get_discord_guild_id_from_interaction(interaction: &ComponentInteraction) -> Option<String> {
    interaction.guild_id.map(|id| id.to_string())
}

pub async fn send_success_response(
    ctx: &SerenityContext,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    interaction
        .create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new()
                .content("Success!")
                .components(vec![]),
        )
        .await?;

    Ok(())
}

pub async fn send_modal_error_response(
    ctx: &SerenityContext,
    interaction: &ModalInteraction,
    error_message: &str,
) -> Result<(), Error> {
    match interaction
        .create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new()
                .content(error_message)
                .components(vec![]),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::from(err)),
    }
}

pub async fn send_component_error_response(
    ctx: &SerenityContext,
    interaction: &ComponentInteraction,
    error_message: &str,
) -> Result<(), Error> {
    match interaction
        .create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new()
                .content(error_message)
                .components(vec![]),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::from(err)),
    }
}
