mod commands;
mod handler;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, FullEvent};

use service::DbService;

use crate::handler::handle_interaction;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, DbService, Error>;

async fn on_error(error: poise::FrameworkError<'_, DbService, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Command error: {} {:?}", ctx.command().name, error);
        }
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Setup error: {:?}", error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error handling error: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token =
        std::env::var("DISCORD_BOT_TOKEN").expect("Expected DISCORD_BOT_TOKEN in .env file");

    let prefix = std::env::var("DISCORD_BOT_PREFIX").unwrap_or("!".to_string());

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::register_user(),
            commands::register_org(),
            commands::register_member(),
            commands::register_members(),
            commands::create_season(), //commands::register_members()
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(prefix),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Command invoked: {}", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Command completed: {}", ctx.command().qualified_name);
            })
        },
        skip_checks_for_owners: false,
        event_handler: |ctx, event, framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                match event {
                    FullEvent::InteractionCreate { interaction } => {
                        let db_service = framework.user_data;
                        handle_interaction(db_service, ctx, interaction).await;
                    }
                    _ => {}
                }
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Bot is ready!");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let mut db_service = DbService::from_env()?;
                db_service.connect().await?;
                Ok(db_service)
            })
        })
        .options(options)
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
