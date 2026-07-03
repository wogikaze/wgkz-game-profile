mod commands;
mod config;
mod model;
mod profile;
mod sources;
mod store;

use anyhow::Result;
use commands::Data;
use config::Config;
use poise::serenity_prelude as serenity;
use std::path::PathBuf;
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cfg = Config::from_env()?;
    let state_path = PathBuf::from(&cfg.state_path);

    // Cron scheduler: periodic refresh.
    let cron_cfg = cfg.clone();
    let cron_state = state_path.clone();
    let cron_expr = cfg.widget_cron.clone();
    let sched = JobScheduler::new().await?;
    let job = Job::new_async(cron_expr.as_str(), move |_uuid, _l| {
        let cfg = cron_cfg.clone();
        let state = cron_state.clone();
        Box::pin(async move {
            if let Err(e) = commands::refresh_silent(&cfg, &state).await {
                tracing::error!("cron refresh failed: {e}");
            }
        })
    })?;
    sched.add(job).await?;
    sched.start().await?;
    tracing::info!("cron scheduler started: {cron_expr}");

    // Discord bot via poise.
    let intents = serenity::GatewayIntents::non_privileged();
    let token = cfg.discord_bot_token.clone();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            let cfg = cfg.clone();
            let state_path = state_path.clone();
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                tracing::info!("{} ready", ready.user.name);
                Ok(Data { cfg, state_path })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;
    client.start_autosharded().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_cron_scheduler::Job;

    #[tokio::test]
    async fn cron_expression_parses() {
        let expr = "0 */30 * * * *";
        let job = Job::new_async(expr, |_, _| Box::pin(async {}));
        assert!(job.is_ok(), "cron parse failed for {expr:?}: {:?}", job.err());
    }
}
