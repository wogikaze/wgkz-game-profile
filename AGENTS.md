# wgkz-game-profile

Discord profile widget updater (Rust). Fetches stats from multiple sources and
pushes them to the Discord user application profile via the REST API.

## Sources

- **Mahjong Soul 4麻 / 3麻** — amae-koromo API (`pl4` / `pl3`).
  - 4麻 mode: `16.12.9.15.11.8`, 3麻 mode: `26.24.22.25.23.21`.
  - Level format: `{name}{stars} {score}/{max}`. Stars = `★`×minor (1..3).
    魂天 has no stars / no max.
  - Level max by major: 初心=200, 雀士=600, 雀傑=1200, 雀豪=2400, 雀聖=4800, 魂天=none.
  - Rank icon: `{base}/{3|4}_{major}.png`.
- **GitHub commits** — GraphQL `contributionsCollection`.
  - `t2` = yesterday's commits, `t3` = this year's commits.
- **TickTick focus** — `pomodoros/statistics/dist/{yyyyMMdd}/{yyyyMMdd}`.
  - `t1` = sum of `tagDurations` minutes converted to milliseconds for Discord display.
- **AtCoder** — `history/json` (algo) and `?contestType=heuristic`.
  - `t4` = algo rating, `t5` = heuristic rating (latest `NewRating`).
- **Book** — manual. Stored in `data/state.json`. Updated via `/widget update`.

## Commands

- `/widget refresh` — fetch all sources and PATCH the profile.
- `/widget update field: book value: <text>` — update a manual field, then refresh.

Owner-only (restricted to `DISCORD_USER_ID`).

## Cron

`tokio-cron-scheduler` runs `WIDGET_CRON` (default `0 */30 * * * *`, every 30 min, 6-field cron: sec min hour dom month dow).

## Build / Run

```sh
cp .env.example .env   # fill in real values
cargo run --release
```

## Notes

- `aa` field is a static placeholder (`WIDGET_AA`, default `360000`) — adjust if needed.
- TickTick `tagDurations` unit is minutes; Discord duration display expects milliseconds.
