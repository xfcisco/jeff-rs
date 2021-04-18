#![feature(option_insert)]
use dashmap::DashMap;
use std::env;

use serenity :: {
    model::prelude::*,
    prelude::*,
    Client,
    async_trait
};

static mut FetchTable: DashMap<UserId, String> = DashMap::<UserId, String>::new();

#[tokio::main]
async fn main() {
    let mut bot = Client::builder(&env::var("DISCORD_TOKEN").expect("token"))
        .event_handler(Handler)
        .await
        .expect("bot creation failed.");
   
    if let Err(why) = bot.start().await {
        println!("BOT_ERROR: {}", why);
    };
}


struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} Bot started.", ready.user.name);
        ctx.dnd().await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "./fetch" => {
                let img_url = FetchTable.get(&msg.author.id).unwrap();
                msg.channel_id.send_message(&ctx.http, |m| m
                    .embed(|e| e
                        .title(format!("fetch for {}", msg.author.name))
                        .image(&*img_url)
                )).await;
            },

            
            full_msg if full_msg.starts_with("./setfetch ") => {
                let url = full_msg.split(" ").collect::<Vec<&str>>()[1];
                FetchTable.insert(msg.author.id, url.to_string());
            }
            _ => {}
        };
    }
}
