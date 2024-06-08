use dotenv::dotenv;
use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn shock(
    ctx: Context<'_>,
    #[description = "user to vibrate"]
    user: serenity::User,
    #[description = "strength of shock, from 1 - 10%"]
    #[min = 1]
    #[max = 10]
    strength: Option<u8>,
    #[description = "duration of shock, from 300 - 1,000 ms"]
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

#[poise::command(slash_command, prefix_command)]
async fn vibrate(
    ctx: Context<'_>,
    #[description = "user to vibrate"]
    user: serenity::User,
    #[description = "strength of vibration, from 1 - 99%"]
    #[min = 1]
    #[max = 99]
    strength: Option<u8>,
    #[description = "duration of vibration, from 300 - 2,000 ms"]
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

#[poise::command(slash_command, prefix_command)]
async fn beep(
    ctx: Context<'_>,
    #[description = "user to beep"]
    user: serenity::User,
) -> Result<(), Error> {
    let response = format!("Beeping <@{}>", user.id);
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = dotenv::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![shock(), vibrate(), beep()],
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
