use crate::Error;
use crate::commands::Button;
use poise::serenity_prelude::{
    Context, CreateActionRow, CreateButton, CreateEmbed, CreateMessage, Guild, Mentionable,
    Timestamp, User,
};

pub async fn handle_guild_create(ctx: &Context, guild: &Guild) {
    let guild_id = guild.id.get().to_string();
    if let Ok(owner) = ctx.http.get_user(guild.owner_id).await {
        let _ = send_welcome_dm(ctx, &guild_id, &owner, &guild.name).await;
    }
}

async fn send_welcome_dm(
    ctx: &Context,
    guild_id: &str,
    owner: &User,
    guild_name: &str,
) -> Result<(), Error> {
    let dm_channel = match owner.create_dm_channel(&ctx.http).await {
        Ok(channel) => channel,
        Err(err) => return Err(err.into()),
    };

    let yes_button = Button::RegisterOrgYes {
        org_name: guild_name.to_string(),
        guild_id: guild_id.to_string(),
        owner_id: owner.id.to_string(),
    };
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

    match dm_channel.send_message(&ctx.http, message).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    }

    Ok(())
}
