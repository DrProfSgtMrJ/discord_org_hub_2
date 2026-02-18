use std::str::FromStr;

use crate::commands::{Button, Modal};
use crate::handler::{
    handle_create_season_interaction, handle_register_org_interaction_yes,
    handle_set_current_season_interaction_no, handle_set_current_season_interaction_yes,
};
use poise::serenity_prelude::{Context, Interaction};
use service::DbService;

pub async fn handle_interaction(db_service: &DbService, ctx: &Context, interaction: &Interaction) {
    match interaction {
        Interaction::Modal(inter) => {
            if let Ok(Modal::CreateSeason) = Modal::from_str(&inter.data.custom_id) {
                let _ = handle_create_season_interaction(db_service, ctx, inter).await;
                println!("Got Create Season");
            }
            //println!("Modal interaction received: {}", inter.)
        }
        Interaction::Component(inter) => {
            let custom_id = &inter.data.custom_id;
            println!("Component interaction received: {}", custom_id);

            match Button::from_str(custom_id) {
                Ok(Button::SetAsCurrentSeasonYes { season_uuid }) => {
                    let _ = handle_set_current_season_interaction_yes(
                        db_service,
                        season_uuid.as_str(),
                        ctx,
                        inter,
                    )
                    .await;
                }
                Ok(Button::SetAsCurrentSeasonNo { season_uuid }) => {
                    let _ = handle_set_current_season_interaction_no(
                        db_service,
                        season_uuid.as_str(),
                        ctx,
                        inter,
                    )
                    .await;
                }
                Ok(Button::RegisterOrgYes {
                    org_name,
                    guild_id,
                    owner_id,
                }) => {
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
                Ok(Button::RegisterOrgNo) => {}
                _ => {}
            }
        }
        _ => {}
    }
}
