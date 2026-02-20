use std::str::FromStr;

use crate::BotData;
use crate::components::{Button, ComponentId, Modal, SelectMenu};
use crate::handler::{
    handle_add_members_to_season_interaction, handle_create_season_interaction,
    handle_register_org_interaction_yes, handle_set_current_season_interaction_no,
    handle_set_current_season_interaction_yes,
};
use poise::serenity_prelude::{Context, Interaction};
//use service::DbService;

pub async fn handle_interaction(bot_data: &BotData, ctx: &Context, interaction: &Interaction) {
    let db_service = &bot_data.db_service;

    match interaction {
        Interaction::Modal(inter) => {
            if let Ok(Modal::CreateSeason) = Modal::from_str(&inter.data.custom_id)
                && let Err(err) =
                    handle_create_season_interaction(&bot_data.interaction_cache, ctx, inter).await
            {
                println!("Error handling create season interaction: {}", err);
            }
        }
        Interaction::Component(inter) => {
            let custom_id = &inter.data.custom_id;
            println!("Component interaction received: {}", custom_id);

            match ComponentId::from_str(custom_id) {
                Ok(ComponentId::Button(button)) => match button {
                    Button::SetAsCurrentSeasonYes { season_uuid } => {
                        let _ = handle_set_current_season_interaction_yes(
                            db_service,
                            season_uuid.as_str(),
                            ctx,
                            inter,
                        )
                        .await;
                    }
                    Button::SetAsCurrentSeasonNo { season_uuid } => {
                        let _ = handle_set_current_season_interaction_no(
                            db_service,
                            season_uuid.as_str(),
                            ctx,
                            inter,
                        )
                        .await;
                    }
                    Button::RegisterOrgYes {
                        org_name,
                        guild_id,
                        owner_id,
                    } => {
                        let _ = handle_register_org_interaction_yes(
                            db_service,
                            ctx,
                            inter,
                            org_name.as_str(),
                            guild_id.as_str(),
                            owner_id.as_str(),
                        )
                        .await;
                    }
                    Button::RegisterOrgNo => {}
                    _ => {}
                },
                Ok(ComponentId::SelectMenu(select_menu)) => match select_menu {
                    SelectMenu::MemberSelectMenu { season_uuid } => {
                        if let Err(err) = handle_add_members_to_season_interaction(
                            db_service,
                            season_uuid.as_str(),
                            ctx,
                            inter,
                        )
                        .await
                        {
                            println!("Error with Member Select Menu: {}", err)
                        }
                    }
                    SelectMenu::SeasonTypeSelectMenu => {
                        todo!()
                    }
                },
                Err(err) => {
                    println!("Error parsing component ID: {}", err);
                }
            }
        }
        _ => {}
    }
}
