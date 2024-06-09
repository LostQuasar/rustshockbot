use dotenv::dotenv;
use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn shock(
    ctx: Context<'_>,
    #[description = "user to vibrate"] user: serenity::User,
    #[description = "strength of shock, from 1 - 100%"]
    #[min = 1]
    #[max = 100]
    strength: Option<u8>,
    #[description = "duration of shock, from 300 - 30,000 ms"]
    #[min = 300]
    #[max = 1000]
    duration: Option<u16>,
) -> Result<(), Error> {
    let str = strength.unwrap_or_else(|| 2);
    let dur = duration.unwrap_or_else(|| 300);
    let response = format!("Shocking <@{}> at {}% for {}ms", user.id, str, dur);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn vibrate(
    ctx: Context<'_>,
    #[description = "user to vibrate"] user: serenity::User,
    #[description = "strength of vibration, from 1 - 100%"]
    #[min = 1]
    #[max = 9999]
    strength: Option<u8>,
    #[description = "duration of vibration, from 300 - 30,000 ms"]
    #[min = 300]
    #[max = 2000]
    duration: Option<u16>,
) -> Result<(), Error> {
    let str = strength.unwrap_or_else(|| 10);
    let dur = duration.unwrap_or_else(|| 300);
    let response = format!("Vibrating <@{}> at {}% for {}ms", user.id, str, dur);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn beep(
    ctx: Context<'_>,
    #[description = "user to beep"] user: serenity::User,
) -> Result<(), Error> {
    let response = format!("Beeping <@{}>", user.id);
    ctx.say(response).await?;
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        poise::FrameworkError::ArgumentParse { error, .. } => panic!("Failed to parse argument: {:?}", error),
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = dotenv::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![shock(), vibrate(), beep()],
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
