use std::time::Duration;

use super::common::is_unique_violation;
use poise::serenity_prelude::{
    self, ComponentInteractionDataKind, CreateActionRow, CreateInteractionResponseMessage,
    CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, Member,
};
use service::UserService;

use crate::{Context, Error};

/// Command to register a new user
///
/// Enter !register_user @<member> <timezone>
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn register_user(
    ctx: Context<'_>,
    #[description = "The member to register"] member: Member,
    #[description = "The timezone of the user (e.g. PST, EST, etc.)"] timezone: Option<String>,
) -> Result<(), Error> {
    //Adds a new User to the database
    let new_user = member.user;
    let display_name = new_user.display_name();

    let discord_id = new_user.id.get().to_string();
    let avatar_url = ctx.author().avatar_url();

    let db_service = ctx.data();

    match db_service
        .create_user(discord_id.as_str(), display_name, timezone, avatar_url)
        .await
    {
        Ok(user) => {
            // Do something with the created user
            ctx.reply(format!(
                "User {} registered successfully!",
                user.display_name
            ))
            .await?;
            Ok(())
        }
        Err(err) => {
            // Handle the error
            if is_unique_violation(&err) {
                ctx.reply(format!(
                    "User with Discord ID {} already exists",
                    discord_id
                ))
                .await?;
            } else {
                ctx.reply(format!("An unexpected error occurred: {}", err))
                    .await?;
            }
            Err(err.into())
        }
    }
}

/// Test command
///
/// !pick_color
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn pick_color(ctx: Context<'_>) -> Result<(), Error> {
    let options = vec![
        CreateSelectMenuOption::new("Red", "red").description("The color red"),
        CreateSelectMenuOption::new("Green", "green").description("The color green"),
        CreateSelectMenuOption::new("Blue", "blue").description("The color blue"),
    ];

    let select_menu =
        CreateSelectMenu::new("color_picker", CreateSelectMenuKind::String { options })
            .placeholder("Choose your favorite color!");
    let components = vec![CreateActionRow::SelectMenu(select_menu)];

    ctx.send(
        poise::CreateReply::default()
            .content("What's your favorite color?")
            .components(components),
    )
    .await?;

    while let Some(interaction) = serenity_prelude::ComponentInteractionCollector::new(ctx)
        .filter(move |interaction| interaction.data.custom_id == "color_picker")
        .timeout(Duration::from_secs(60))
        .await
    {
        if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
            let color = &values[0];

            let response = match color.as_str() {
                "red" => "You chose red!",
                "green" => "You chose green!",
                "blue" => "You chose blue!",
                _ => "Invalid color!",
            };

            interaction
                .create_response(
                    ctx.http(),
                    serenity_prelude::CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .content(response)
                            .components(vec![]),
                    ),
                )
                .await?;
            break;
        }
    }
    Ok(())
}

#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn test_button_multiple(ctx: Context<'_>) -> Result<(), Error> {
    let button1 = serenity_prelude::CreateButton::new("Test Button")
        .style(serenity_prelude::ButtonStyle::Primary)
        .label("Test Button");

    let button2 = serenity_prelude::CreateButton::new("Test Button 2")
        .style(serenity_prelude::ButtonStyle::Secondary)
        .label("Test Button 2");

    ctx.send(
        poise::CreateReply::default()
            .components(vec![CreateActionRow::Buttons(vec![button1, button2])]),
    )
    .await?;
    Ok(())
}
