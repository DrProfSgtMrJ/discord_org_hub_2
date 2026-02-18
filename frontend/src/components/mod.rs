//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod bot_invite;
mod hero;
mod hero_header;
pub use bot_invite::BotInvite;
pub use hero::Hero;
pub use hero_header::HeroHeader;
