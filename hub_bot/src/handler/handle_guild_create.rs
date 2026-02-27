use crate::Error;
use crate::components::Button;
use poise::serenity_prelude::{
    ChannelId, ComponentInteractionCollector, Context, CreateActionRow, CreateButton, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateMessage, Guild,
    Mentionable, MessageId, Timestamp, User, UserId,
};
use service::{DbService, OrgService, UserService};
use std::str::FromStr;
use std::time::Duration;

pub async fn handle_guild_create(db_service: &DbService, ctx: &Context, guild: &Guild) {
    let guild_id = guild.id.get().to_string();
    if let Ok(owner) = ctx.http.get_user(guild.owner_id).await {
        let _ = send_welcome_dm(db_service, ctx, &guild_id, &owner, &guild.name).await;
    }
}

async fn send_welcome_dm(
    db_service: &DbService,
    ctx: &Context,
    guild_id: &str,
    owner: &User,
    guild_name: &str,
) -> Result<(), Error> {
    let dm_channel = match owner.create_dm_channel(&ctx.http).await {
        Ok(channel) => channel,
        Err(err) => return Err(err.into()),
    };

    let yes_button = Button::RegisterOrgYes;
    let no_button = Button::RegisterOrgNo;

    let create_yes_button: CreateButton = yes_button.into();
    let create_no_button: CreateButton = no_button.into();
    let welcome_embed = CreateEmbed::new()
        .title("Welcome to Discord Org Hub!")
        .description(format!(
            "Hi {}! I've been added to your server **{}**\n\n
            I can help you manage your organization, seasons, and memers!\n\n
            **To get started:**\n
            1. Use `/register_org @{} <your_org_name>` in your server\n
            2. Create seasons with `/create_season`\n
            3. Register members with `/register_members @member...`\n
            4. Add the members to a season with `/add_to_season <season_id> @<member> <placement>`\n\n
            Click Yes to register {} now!",
            owner.name,
            guild_name,
            owner.mention(),
            guild_name
        ))
        .color(0x00FF00)
        .timestamp(Timestamp::now());
    let action_row = CreateActionRow::Buttons(vec![create_yes_button, create_no_button]);
    let message = CreateMessage::new()
        .embed(welcome_embed)
        .components(vec![action_row]);

    let sent_message = dm_channel.send_message(&ctx.http, message).await?;
    handle_register_org_interaction(
        db_service,
        ctx,
        owner.id,
        dm_channel.id,
        sent_message.id,
        guild_name,
        guild_id,
    )
    .await?;

    Ok(())
}

async fn handle_register_org_interaction(
    db_service: &DbService,
    ctx: &Context,
    author_id: UserId,
    dm_channel_id: ChannelId,
    sent_message_id: MessageId,
    guild_name: &str,
    guild_id: &str,
) -> Result<(), Error> {
    let yes_button_id = Button::RegisterOrgYes.id();
    let no_button_id = Button::RegisterOrgNo.id();

    let Some(interaction) = ComponentInteractionCollector::new(ctx)
        .author_id(author_id)
        .channel_id(dm_channel_id)
        .message_id(sent_message_id)
        .custom_ids(vec![yes_button_id.to_string(), no_button_id.to_string()])
        .timeout(Duration::from_secs(300))
        .await
    else {
        return Ok(());
    };

    interaction
        .create_response(ctx, CreateInteractionResponse::Acknowledge)
        .await?;

    match Button::from_str(interaction.data.custom_id.as_str()) {
        Ok(Button::RegisterOrgNo) => {
            interaction
                .create_followup(
                    &ctx.http,
                    CreateInteractionResponseFollowup::new()
                        .content("No problem. You can use `/register_org` any time."),
                )
                .await?;
        }
        Ok(Button::RegisterOrgYes) => {
            if let Some(owner_user) = db_service
                .get_user_by_discord_id(&author_id.to_string())
                .await?
            {
                match db_service
                    .create_org(guild_name, guild_id, owner_user.id, None)
                    .await
                {
                    Ok(_) => {
                        interaction
                            .create_followup(
                                &ctx.http,
                                CreateInteractionResponseFollowup::new()
                                    .content("Success! Your org was registered."),
                            )
                            .await?;
                    }
                    Err(_) => {
                        interaction
                            .create_followup(
                                &ctx.http,
                                CreateInteractionResponseFollowup::new()
                                    .content("Failed to register org"),
                            )
                            .await?;
                    }
                }
            } else {
                interaction
                    .create_followup(
                        &ctx.http,
                        CreateInteractionResponseFollowup::new()
                            .content("Please register your account first (do '/join')"),
                    )
                    .await?;
            }
        }
        _ => {
            interaction
                .create_followup(
                    &ctx.http,
                    CreateInteractionResponseFollowup::new().content("Unexpected error"),
                )
                .await?;
        }
    }
    Ok(())
}
