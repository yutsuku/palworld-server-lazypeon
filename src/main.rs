mod commands;

use std::env;

use serenity::async_trait;
use serenity::builder::EditInteractionResponse;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use tracing::{debug, info};


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            info!(
                id = command.id.to_string(),
                info = "Received command interaction",
                command = command.data.name,
                user_id = &command.user.id.to_string(),
                user_name = &command.user.name,
                channel_id =  &command.channel_id.to_string(),
                guild_id =  &command.guild_id.unwrap().to_string()
            );

            let root_dir = env::var("SERVER_ROOT").expect("game server root directory");

            let _ = command.defer_ephemeral(&ctx.http).await;

            let lazy_content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "start" => Some(commands::start::run(&ctx, &root_dir).await),
                "stop" => Some(commands::stop::run(&ctx, &root_dir).await),
                "logs" => Some(commands::logs::run(&ctx, &root_dir).await),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(lazy_content) = lazy_content {
                debug!(
                    id = command.id.to_string(),
                    response = lazy_content
                );
                let builder = EditInteractionResponse::new()
                    .content(lazy_content);
                if let Err(why) = command.edit_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        info!("Serving {} guild(s)", ready.guilds.len());

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
                commands::start::register(),
                commands::stop::register(),
                commands::logs::register(),
            ])
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");

        let guild_command =
            Command::create_global_command(&ctx.http, commands::wonderful_command::register())
                .await;

        println!("I created the following global slash command: {guild_command:#?}");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("sup");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}