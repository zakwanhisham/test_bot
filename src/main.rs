use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting test_bot...");

    let bot = Bot::from_env().auto_send();

    // teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
    //     bot.send_dice(message.chat.id).await?;
    //     respond(())
    // })
    // .await;

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These command are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UserNameAndAge { username: String, age: u8 },
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(message.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UserNameAndAge { username, age } => {
            bot.send_message(
                message.chat.id,
                format!("Your username is @{username} and your age is {age}."),
            )
            .await?
        }
    };
    Ok(())
}
