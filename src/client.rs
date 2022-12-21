use std::{
    env,
    time::Duration,
};

use poise::{
    serenity_prelude::{
        Activity,
        Error as SerenityError,
        GatewayIntents as Intents,
    },
    EditTracker,
    Framework,
    FrameworkOptions,
    PrefixFrameworkOptions,
};

use crate::{
    commands,
    data::Data,
    handlers::{
        event,
        on_error,
        post_command,
        pre_command,
    },
    types::Error,
};

fn prefix_options() -> PrefixFrameworkOptions<Data, Error> {
    PrefixFrameworkOptions {
        dynamic_prefix: Some(|_ctx| {
            // TODO
            Box::pin(async move { Ok(Some(">".to_owned())) })
        }),
        edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(60))),
        ..Default::default()
    }
}

fn framework_options() -> FrameworkOptions<Data, Error> {
    FrameworkOptions {
        commands: commands::COMMANDS,
        on_error: on_error::handler,
        pre_command: pre_command::handler,
        post_command: post_command::handler,
        skip_checks_for_owners: true,
        allowed_mentions: None,
        event_handler: event::handler,
        prefix_options: prefix_options(),
        ..Default::default()
    }
}

pub async fn run() -> Result<(), SerenityError> {
    let builder = Framework::builder()
        .setup(move |ctx, _, _| {
            Box::pin(async move {
                ctx.set_activity(Activity::listening("to music")).await;
                Ok(Data {})
            })
        })
        .options(framework_options())
        .token(env::var("BOT_TOKEN").expect("BOT_TOKEN not set."))
        .intents(Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT);

    builder.build().await?.start_autosharded().await
}
