use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use rzap::{api::OpenShockAPI, data_type::ControlType};

struct Data {
    openshock: OpenShockAPI,
    id: String
}
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
    #[max = 30000]
    duration: Option<u16>,
) -> Result<(), Error> {
    let stren = strength.unwrap_or_else(|| 2);
    let dur = duration.unwrap_or_else(|| 300);
    let resp = ctx.data().openshock.post_control(ctx.data().id.clone(), ControlType::Shock, stren, dur, None);
    let response = match resp.await {
        Ok(_) => format!("Shocking <@{}> at {}% for {}ms", user.id, stren, dur),
        Err(_) => "Unable to send shock".to_string()
    };
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn vibrate(
    ctx: Context<'_>,
    #[description = "user to vibrate"] user: serenity::User,
    #[description = "strength of vibration, from 1 - 100%"]
    #[min = 1]
    #[max = 100]
    strength: Option<u8>,
    #[description = "duration of vibration, from 300 - 30,000 ms"]
    #[min = 300]
    #[max = 30000]
    duration: Option<u16>,
) -> Result<(), Error> {
    let stren = strength.unwrap_or_else(|| 1);
    let dur = duration.unwrap_or_else(|| 300);
    let resp = ctx.data().openshock.post_control(ctx.data().id.clone(), ControlType::Vibrate, stren, dur, None);
    let response = match resp.await {
        Ok(_) => format!("Vibrating <@{}> at {}% for {}ms", user.id, stren, dur),
        Err(_) => "Unable to send vibration".to_string()
    };
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn beep(
    ctx: Context<'_>,
    #[description = "user to beep"] user: serenity::User,
) -> Result<(), Error> {
    let resp = ctx.data().openshock.post_control(ctx.data().id.clone(), ControlType::Sound, 1, 300, None);
    let response = match resp.await {
        Ok(_) => format!("Beeping <@{}>", user.id),
        Err(_) => "Unable to send beep".to_string()
    };
    ctx.say(response).await?;
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        poise::FrameworkError::CommandStructureMismatch { description, ctx, .. } => panic!("Command structure mismatch: {:?} in command {}", description, ctx.command().name),
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
    let discord_token = dotenv::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let openshock_key = dotenv::var("OPENSHOCK_KEY").expect("missing OPENSHOCK_KEY");
    let test_id = dotenv::var("TEST_ID").expect("missing TEST_ID");
    let intents = serenity::GatewayIntents::non_privileged();
    let openshock = OpenShockAPI::new(None, openshock_key);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![shock(), vibrate(), beep()],
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {openshock, id: test_id})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;
        
    client.unwrap().start().await.unwrap();
}
