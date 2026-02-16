use super::common::is_unique_violation;
use service::UserService;

use crate::{Context, Error};
use poise::serenity_prelude::{Member, User};

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
    register_user_internal(ctx, new_user, timezone).await
}

/// Command to register a user to org discord hub!
///
/// Enter !register_user @<member>  <timezone>
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn register_user(
    ctx: Context<'_>,
    #[description = "The member to register"] member: Member,
    #[description = "Your timezone (e.g. PST, EST, etc.)"] timezone: Option<String>,
) -> Result<(), Error> {
    let user = member.user;
    register_user_internal(ctx, &user, timezone).await
}

async fn register_user_internal(
    ctx: Context<'_>,
    user: &User,
    timezone: Option<String>,
) -> Result<(), Error> {
    let display_name = user.display_name();

    let discord_id = user.id.get().to_string();
    let avatar_url = user.avatar_url();

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
