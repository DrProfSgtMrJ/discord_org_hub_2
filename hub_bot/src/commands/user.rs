use super::common::is_unique_violation;
use service::UserService;

use crate::{Context, Error};

/// Command to register yourself to org discord hub!
///
/// Enter !join  <timezone>
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Your timezone (e.g. PST, EST, etc.)"] timezone: Option<String>,
) -> Result<(), Error> {
    //Adds a new User to the database
    let new_user = ctx.author();
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
