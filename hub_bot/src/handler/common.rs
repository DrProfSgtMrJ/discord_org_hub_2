use crate::Error;
use poise::serenity_prelude::{
    ComponentInteraction, Context as SerenityContext, CreateInteractionResponse,
    CreateInteractionResponseMessage, ModalInteraction,
};

pub async fn send_success_response(
    ctx: &SerenityContext,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Success!")
                    .components(vec![]),
            ),
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
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(error_message)
                    .components(vec![]),
            ),
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
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(error_message)
                    .components(vec![]),
            ),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::from(err)),
    }
}
