use std::str::FromStr;

use crate::commands::{Button, Modal};
use crate::handler::{
    handle_create_season_interaction, handle_set_current_season_interaction_no,
    handle_set_current_season_interaction_yes,
};
use poise::serenity_prelude::{Context, Interaction};
use service::DbService;

pub async fn handle_interaction(db_service: &DbService, ctx: &Context, interaction: &Interaction) {
    match interaction {
        Interaction::Modal(inter) => {
            match Modal::from_str(&inter.data.custom_id) {
                Ok(Modal::CreateSeason) => {
                    // Handle CreateSeason modal interaction
                    let _ = handle_create_season_interaction(db_service, ctx, inter).await;
                    println!("Got Create Season");
                }
                _ => {}
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
                _ => {}
            }
        }
        _ => {}
    }
}
