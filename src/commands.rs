use crate::config::Config;
use crate::store::State;
use crate::{profile, sources};
use std::path::{Path, PathBuf};

pub struct Data {
    pub cfg: Config,
    pub state_path: PathBuf,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn is_owner(ctx: &Context<'_>) -> bool {
    ctx.author().id.get() == ctx.data().cfg.discord_user_id
}

async fn do_refresh(ctx: &Context<'_>) -> Result<String, Error> {
    let data = ctx.data();
    let state = State::load(&data.state_path)?;
    let stats = sources::fetch_all(&data.cfg, state.book).await?;
    profile::push_profile(&data.cfg, &stats).await?;
    Ok(format!(
        "Updated: 4ma[{}] 3ma[{}] focus={} commits(y/y)={}/{} atcoder(a/h)={}/{} book={}",
        stats.majsoul_4ma.format(),
        stats.majsoul_3ma.format(),
        stats.focus_ms / 60_000,
        stats.commits_yesterday,
        stats.commits_year,
        stats.atcoder_algo,
        stats.atcoder_heuristic,
        if stats.book.is_empty() { "(none)" } else { &stats.book },
    ))
}

/// Refresh the widget from all sources.
#[poise::command(slash_command)]
pub async fn refresh(ctx: Context<'_>) -> Result<(), Error> {
    if !is_owner(&ctx) {
        ctx.say("Not allowed.").await?;
        return Ok(());
    }
    ctx.defer().await?;
    match do_refresh(&ctx).await {
        Ok(msg) => {
            ctx.say(msg).await?;
        }
        Err(e) => {
            ctx.say(format!("Error: {e}")).await?;
        }
    }
    Ok(())
}

/// Update a manual field, then refresh.
#[poise::command(slash_command)]
pub async fn update(
    ctx: Context<'_>,
    #[description = "Field to update"] field: UpdateField,
    #[description = "New value"] value: String,
) -> Result<(), Error> {
    if !is_owner(&ctx) {
        ctx.say("Not allowed.").await?;
        return Ok(());
    }
    ctx.defer().await?;
    let data = ctx.data();
    let mut state = State::load(&data.state_path)?;
    match field {
        UpdateField::Book => state.book = value.clone(),
    }
    state.save(&data.state_path)?;
    match do_refresh(&ctx).await {
        Ok(msg) => {
            ctx.say(format!("{msg}\nUpdated field `{field:?}` = {value}"))
                .await?;
        }
        Err(e) => {
            ctx.say(format!("Saved but refresh failed: {e}"))
                .await?;
        }
    }
    Ok(())
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum UpdateField {
    #[name = "book"]
    Book,
}

/// Parent command grouping widget operations.
#[poise::command(slash_command, subcommands("refresh", "update"))]
pub async fn widget(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use `/widget refresh` or `/widget update`.").await?;
    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![widget()]
}

#[allow(dead_code)]
pub async fn refresh_silent(cfg: &Config, state_path: &Path) -> Result<(), Error> {
    let state = State::load(state_path)?;
    let stats = sources::fetch_all(cfg, state.book).await?;
    profile::push_profile(cfg, &stats).await?;
    Ok(())
}

#[allow(dead_code)]
pub fn make_data(cfg: Config, state_path: PathBuf) -> Data {
    Data { cfg, state_path }
}
