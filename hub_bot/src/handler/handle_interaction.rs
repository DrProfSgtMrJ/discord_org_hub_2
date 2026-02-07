use std::str::FromStr;

use crate::commands::Modal;
use crate::handler::handle_create_season_interaction;
use poise::serenity_prelude::{Context, Interaction};
use service::DbService;

pub async fn handle_interaction(db_service: &DbService, ctx: &Context, interaction: &Interaction) {
    match interaction {
        Interaction::Modal(inter) => {
            match Modal::from_str(&inter.data.custom_id) {
                Ok(Modal::CreateSeason) => {
                    // Handle CreateSeason modal interaction
                    handle_create_season_interaction(db_service, ctx, inter).await;
                    println!("Got Create Season");
                }
                _ => {}
            }
            //println!("Modal interaction received: {}", inter.)
        }
        Interaction::Component(inter) => {
            println!("Component interaction received: {}", inter.id.get())
        }
        _ => {}
    }
}
