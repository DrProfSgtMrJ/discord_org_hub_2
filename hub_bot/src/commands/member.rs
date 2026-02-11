use super::common::is_unique_violation;
use poise::serenity_prelude::CreateActionRow;
use poise::serenity_prelude::CreateInteractionResponse;
use poise::serenity_prelude::CreateInteractionResponseMessage;
use poise::serenity_prelude::CreateModal;
use poise::serenity_prelude::CreateSelectMenu;
use poise::serenity_prelude::CreateSelectMenuOption;
use poise::serenity_prelude::Member;
use service::MemberService;
use service::OrgService;
use service::SeasonService;
use service::UserService;

use super::components::{Modal, SelectMenu};
use crate::commands::common::get_discord_build_id_from_context;
use crate::{Context, Error};
use uuid::Uuid;

/// Command to register a member to an organization
///
/// Enter !register_member @member to register the mentioned member to the organization associated with the current guild
#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn register_member(
    ctx: Context<'_>,
    #[description = "The member to register"] member: Member,
) -> Result<(), Error> {
    if let Some(org_id) = ctx.guild_id() {
        let org_discord_id = org_id.get().to_string();
        let discord_user_id = member.user.id.get().to_string();

        let db_service = ctx.data();

        if let Some(org) = db_service.get_org_by_discord_id(&org_discord_id).await? {
            if let Some(user) = db_service.get_user_by_discord_id(&discord_user_id).await? {
                let org_uuid = org.id;
                let user_uuid = user.id;
                match db_service.create_member(user_uuid, org_uuid).await {
                    Ok(_) => {
                        ctx.reply(format!(
                            "Member {} registered successfully!",
                            user.display_name
                        ))
                        .await?;
                    }
                    Err(err) => {
                        if is_unique_violation(&err) {
                            ctx.reply(format!(
                                "Member with Discord ID {} already exists in the organization",
                                discord_user_id
                            ))
                            .await?;
                        } else {
                            ctx.reply("An unexpected database error occurred").await?;
                        }
                    }
                }
            } else {
                ctx.reply(format!(
                    "User with Discord ID {} not found. Please register as a user first.",
                    discord_user_id
                ))
                .await?;
            }
        } else {
            ctx.reply(format!(
                "Organization with Discord ID {} not found. Please register the organization first.",
                org_discord_id
            )).await?;
        }
    }
    Ok(())
}

#[poise::command(prefix_command, track_edits, owners_only, slash_command)]
pub async fn register_members(ctx: Context<'_>) -> Result<(), Error> {
    setup_register_members_modal(ctx).await
}

async fn setup_register_members_modal(ctx: Context<'_>) -> Result<(), Error> {
    let db_service = ctx.data();
    if let Some(discord_id) = get_discord_build_id_from_context(&ctx) {
        match db_service
            .get_org_by_discord_id(discord_id.as_str())
            .await?
        {
            Some(org) => {
                // Handle the case when the organization is found
                let org_id = org.id;
                let seasons = db_service.get_seasons_by_org_id(org_id).await?;
                if seasons.is_empty() {
                    ctx.reply(
                        "No seasons found for this organization. Please create a season first.",
                    )
                    .await?;
                    return Ok(());
                }
                //let modal: CreateModal = Modal::AddMembersToSeason.into();
                let season_select: CreateSelectMenu = SelectMenu::SeasonSelectMenu(seasons).into();
                let season_select_row = CreateActionRow::SelectMenu(season_select);
                if let poise::Context::Application(app_ctx) = ctx {
                    app_ctx
                        .interaction
                        .create_response(
                            &ctx.serenity_context().http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .components(vec![season_select_row]),
                            ),
                        )
                        .await?;
                } else {
                    ctx.reply(
                        "This command only works as a slash command. Please use /create_server",
                    )
                    .await?;
                }
            }
            None => {
                ctx.reply(format!(
                    "Organization with Discord ID {} not found. Please register the organization first.",
                    discord_id
                ))
                .await?;
            }
        }
    }
    Ok(())
}
