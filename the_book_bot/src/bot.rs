use crate::bot::verse::fetch_pages;
use crate::bot::verse::generate_page;
use crate::bot::verse::types::Page;

use teloxide::{prelude::*, utils::command::BotCommands};
use dotenv::dotenv;

mod verse;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "source code")]
    Source,
    #[command(description = "what is my purpose?")]
    Purpose,
    #[command(description = "picks a verse")]
    Verse,
}

pub async fn run() {
    pretty_env_logger::init();
    dotenv().ok();
    
    log::info!("Accessing the scripture...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Source => {
            bot.send_message(msg.chat.id, format!("https://github.com/jasshanK/TheBookBot")).await?
        }
        Command::Purpose => {
            bot.send_message(msg.chat.id, format!("My purpose is to preach the verses of The Book to Oxidise the world.")).await?
        }
        Command::Verse => {
            let mut pages: Vec<Page> = Vec::new();
            let seed = String::from("https://doc.rust-lang.org/stable/book/");
        
            fetch_pages(&seed, &mut pages).await;
            let select: Page = generate_page(&mut pages);

            bot.send_message(msg.chat.id, format!("Teaching: {}\nSource: {}", select.title, seed + &select.url)).await?
        }
    };

    Ok(())
}