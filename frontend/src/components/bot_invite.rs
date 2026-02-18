use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/bot-logo-256.png");
const BOT_INVITE_CSS: Asset = asset!("assets/styling/bot-invite.css");

/// Replace this with your real Discord bot OAuth2 invite URL.
const BOT_INVITE_URL: &str =
    "https://discord.com/oauth2/authorize?client_id=YOUR_CLIENT_ID&scope=bot&permissions=0";

#[component]
pub fn BotInvite() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: BOT_INVITE_CSS,
        }
        a {
            id: "bot-invite",
            href: BOT_INVITE_URL,
            target: "_blank",
            rel: "noopener noreferrer",

            img {
                src: LOGO,
                id: "bot-invite-logo",
                alt: "Discord Org Hub Bot Logo",
                width: "64",
                height: "64",
            }

            div { id: "bot-invite-text",
                span { class: "bot-invite-heading", "Add the Bot to Your Server" }
                span { class: "bot-invite-sub", "Click here to invite Discord Org Hub to your server" }
            }
        }
    }
}
