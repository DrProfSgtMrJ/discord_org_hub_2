use poise::serenity_prelude::Member;
use service::UserService;

use crate::{Context, Error};

#[poise::command(prefix_command, track_edits, owners_only)]
pub async fn register_user(
    ctx: Context<'_>,
    member: Member,
    timezone: Option<String>,
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
            ctx.reply(format!("An unexpected error occurred: {}", err))
                .await?;
            Err(err.into())
        }
    }
}
